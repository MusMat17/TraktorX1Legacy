// Derived from Opa-/x1-mk1-usb2midi (Apache-2.0), modified for TraktorX1Legacy.
use std::fs::File;
use std::path::PathBuf;
use std::time::Duration;

use rusb::{Device, DeviceHandle, UsbContext};

pub fn get_serial_number<T: UsbContext>(
    dev: &Device<T>,
    handle: &DeviceHandle<T>,
) -> Option<String> {
    let timeout = Duration::from_secs(1);
    let descriptor = dev.device_descriptor().ok()?;
    let languages = handle.read_languages(timeout).ok()?;
    let language = *languages.first()?;
    let serial = handle
        .read_serial_number_string(language, &descriptor, timeout)
        .ok()?
        .trim()
        .to_uppercase();

    if serial.is_empty() {
        None
    } else {
        Some(serial)
    }
}

pub fn fallback_device_name<T: UsbContext>(dev: &Device<T>) -> String {
    format!("X1-{:03}-{:03}", dev.bus_number(), dev.address())
}

pub fn hex2bool(hex: u8, bin: &mut [bool; 8]) {
    for (i, value) in bin.iter_mut().enumerate() {
        *value = ((hex >> i) & 1) != 0;
    }
}

pub fn hex2bin(hex: u8, bin: &mut [u8; 8]) {
    for (i, value) in bin.iter_mut().enumerate() {
        *value = (hex >> i) & 1;
    }
}

pub fn knob_to_midi(msb: u8, lsb: u8) -> u8 {
    let raw = (((msb as u16) << 8) | lsb as u16).min(0x0fff);
    ((raw as f32 / 0x0fff as f32) * 127.0).round() as u8
}

pub fn get_yaml_file() -> Result<File, String> {
    let bundle_path = get_yaml_path();
    if let Ok(file) = File::open(&bundle_path) {
        return Ok(file);
    }

    File::open("board.yml").map_err(|err| {
        format!(
            "Failed to open board.yml (bundle path: {}): {err}",
            bundle_path.display()
        )
    })
}

fn get_yaml_path() -> PathBuf {
    let mut path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    path.pop();
    path.pop();
    path.push("Resources");
    path.push("board.yml");
    path
}

#[cfg(test)]
mod tests {
    use super::knob_to_midi;

    #[test]
    fn knob_scaling_is_clamped() {
        assert_eq!(knob_to_midi(0x00, 0x00), 0);
        assert_eq!(knob_to_midi(0x0f, 0xff), 127);
        assert_eq!(knob_to_midi(0xff, 0xff), 127);
    }
}
