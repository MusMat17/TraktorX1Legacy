// Derived from Opa-/x1-mk1-usb2midi (Apache-2.0), modified for TraktorX1Legacy.
#[cfg(target_os = "macos")]
use std::cell::RefCell;

use std::collections::HashMap;
use std::io::Read;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use rusb::{Context, Device, HotplugBuilder, Registration, UsbContext};

#[cfg(target_os = "macos")]
use system_status_bar_macos::{sync_infinite_event_loop, Menu, MenuItem, StatusItem};

use crate::conf::YamlConfig;
use crate::usb_hotplug::HotPlugHandler;
use crate::utils::{fallback_device_name, get_serial_number, get_yaml_file};
use crate::x1_process::X1mk1;

mod conf;
mod usb_hotplug;
mod utils;
mod x1_board;
mod x1_process;

const USB_ID_VENDOR: u16 = 0x17cc;
const USB_ID_PRODUCT: u16 = 0x2305;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--version") {
        println!("TraktorX1Legacy {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.iter().any(|arg| arg == "--diagnose") {
        if let Err(err) = diagnose() {
            eprintln!("[fatal] {err}");
            std::process::exit(1);
        }
        return;
    }

    #[cfg(not(target_os = "macos"))]
    {
        let (sender, _receiver) = mpsc::channel::<HashMap<String, bool>>();
        if let Err(err) = run_bridge(sender) {
            eprintln!("[fatal] {err}");
            std::process::exit(1);
        }
    }

    #[cfg(target_os = "macos")]
    {
        let (sender_menu_bar, receiver_menu_bar) = mpsc::channel::<HashMap<String, bool>>();

        thread::spawn(move || {
            if let Err(err) = run_bridge(sender_menu_bar) {
                eprintln!("[fatal] {err}");
            }
        });

        let status_item = RefCell::new(StatusItem::new("🎛️", build_status_menu(&HashMap::new())));

        sync_infinite_event_loop(receiver_menu_bar, move |x1| {
            status_item.borrow_mut().set_menu(build_status_menu(&x1));
        });
    }
}

#[cfg(target_os = "macos")]
fn build_status_menu(devices: &HashMap<String, bool>) -> Menu {
    let mut entries: Vec<_> = devices.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    let mut items: Vec<MenuItem> = if entries.is_empty() {
        vec![MenuItem::new("Waiting for X1 Mk1…", None, None)]
    } else {
        entries
            .into_iter()
            .map(|(name, connected)| {
                MenuItem::new(
                    format!("{} {}", if *connected { "🟢" } else { "🔴" }, name),
                    None,
                    None,
                )
            })
            .collect()
    };

    // Always rebuild Quit on USB status updates; otherwise it would disappear.
    items.push(MenuItem::new(
        "Quit TraktorX1Legacy",
        Some(Box::new(|| {
            eprintln!("[app] Quit from menu bar");
            std::process::exit(0);
        })),
        None,
    ));
    Menu::new(items)
}

fn load_config() -> Result<YamlConfig, String> {
    let mut file = get_yaml_file()?;
    let mut yaml_content = String::new();
    file.read_to_string(&mut yaml_content)
        .map_err(|e| format!("Failed to read board.yml: {e}"))?;
    serde_yaml::from_str(&yaml_content).map_err(|e| format!("Failed to parse board.yml: {e}"))
}

fn run_bridge(sender_menu_bar: Sender<HashMap<String, bool>>) -> Result<(), String> {
    let yaml_config = load_config()?;

    if !rusb::has_hotplug() {
        return Err("libusb was compiled without hotplug support".to_string());
    }

    let context = Context::new().map_err(|e| format!("USB context: {e:?}"))?;
    let (tx, rx) = mpsc::channel::<Device<Context>>();

    let mut registration: Option<Registration<Context>> = Some(
        HotplugBuilder::new()
            .enumerate(true)
            .vendor_id(USB_ID_VENDOR)
            .product_id(USB_ID_PRODUCT)
            .register(&context, Box::new(HotPlugHandler { sender: tx }))
            .map_err(|e| format!("Register USB hotplug callback: {e:?}"))?,
    );

    let devices = Arc::new(Mutex::new(HashMap::<String, bool>::new()));

    thread::spawn({
        let devices = Arc::clone(&devices);
        let sender_menu_bar = sender_menu_bar.clone();

        move || {
            while let Ok(device) = rx.recv() {
                let handle = match device.open() {
                    Ok(handle) => handle,
                    Err(err) => {
                        eprintln!("[usb] failed to open X1 Mk1: {err:?}");
                        continue;
                    }
                };

                let serial_number = get_serial_number(&device, &handle)
                    .unwrap_or_else(|| fallback_device_name(&device));

                {
                    let mut state = devices.lock().expect("device state mutex poisoned");
                    if state.get(&serial_number) == Some(&true) {
                        eprintln!("[usb] ignoring duplicate arrival for {serial_number}");
                        continue;
                    }
                    state.insert(serial_number.clone(), true);
                    let _ = sender_menu_bar.send(state.clone());
                }

                let devices = Arc::clone(&devices);
                let sender_menu_bar = sender_menu_bar.clone();
                let yaml_config = yaml_config.clone();

                thread::spawn(move || {
                    let result = X1mk1::new(device, handle, serial_number.clone(), yaml_config)
                        .and_then(|mut x1| x1.run());

                    if let Err(err) = result {
                        eprintln!("[bridge] {serial_number}: {err}");
                    }

                    let mut state = devices.lock().expect("device state mutex poisoned");
                    state.insert(serial_number.clone(), false);
                    let _ = sender_menu_bar.send(state.clone());
                });
            }
        }
    });

    loop {
        if let Err(err) = context.handle_events(None) {
            eprintln!("[usb] event loop error: {err:?}");
            if let Some(reg) = registration.take() {
                context.unregister_callback(reg);
            }
            return Err(format!("USB event loop stopped: {err:?}"));
        }
    }
}

fn diagnose() -> Result<(), String> {
    println!("TraktorX1Legacy {}", env!("CARGO_PKG_VERSION"));
    println!("Target: {}", std::env::consts::ARCH);
    println!(
        "Looking for USB {:04x}:{:04x} (Traktor Kontrol X1 Mk1)",
        USB_ID_VENDOR, USB_ID_PRODUCT
    );

    let context = Context::new().map_err(|e| format!("USB context: {e:?}"))?;
    let list = context
        .devices()
        .map_err(|e| format!("Enumerate USB devices: {e:?}"))?;

    let mut found = 0usize;
    for device in list.iter() {
        let descriptor = match device.device_descriptor() {
            Ok(descriptor) => descriptor,
            Err(_) => continue,
        };

        if descriptor.vendor_id() != USB_ID_VENDOR || descriptor.product_id() != USB_ID_PRODUCT {
            continue;
        }

        found += 1;
        let serial = match device.open() {
            Ok(handle) => {
                get_serial_number(&device, &handle).unwrap_or_else(|| fallback_device_name(&device))
            }
            Err(_) => fallback_device_name(&device),
        };

        println!(
            "FOUND: bus={} address={} name={}",
            device.bus_number(),
            device.address(),
            serial
        );
    }

    if found == 0 {
        println!("RESULT: no X1 Mk1 detected");
    } else {
        println!("RESULT: detected {found} X1 Mk1 device(s)");
    }

    Ok(())
}
