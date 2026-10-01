// Derived from Opa-/x1-mk1-usb2midi (Apache-2.0), modified for TraktorX1Legacy.
use std::sync::mpsc;
use std::time::Duration;

use midir::os::unix::{VirtualInput, VirtualOutput};
use midir::{MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use rusb::{Device, DeviceHandle, UsbContext};

use crate::conf::YamlConfig;
use crate::utils::{hex2bin, hex2bool, knob_to_midi};
use crate::x1_board::{ButtonType, X1mk1Board};

const USB_WRITE_FD: u8 = 0x01;
const USB_UNLOCK_FD: u8 = 0x81;
const USB_READ_FD: u8 = 0x84;
const LED_DIM: u8 = 0x05;
const LED_BRIGHT: u8 = 0x7f;
const MIDI_CHANNEL: u8 = 0xb0;
const MIDI_CHANNEL_LED: u8 = 0xb2;
const MIDI_CHANNEL_HOTCUE: u8 = 0xb3;

pub struct X1mk1<T: UsbContext> {
    #[allow(dead_code)]
    device: Device<T>,
    handle: DeviceHandle<T>,
    serial_number: String,
    midi_conn_out: MidiOutputConnection,
    midi_conn_in: Option<MidiInputConnection<()>>,
    board: X1mk1Board,
    usb_buffer: [u8; 24],
    usb_timeout: Duration,
    usb_endpoint: Endpoint,
    led: [u8; 32],
    led_hotcue: [u8; 16],
    shift: u8,
    hotcue: bool,
}

struct Endpoint {
    config: u8,
    interface: u8,
    setting: u8,
    address: u8,
}

impl<T: UsbContext> X1mk1<T> {
    pub fn new(
        device: Device<T>,
        handle: DeviceHandle<T>,
        serial_number: String,
        yaml_config: YamlConfig,
    ) -> Result<Self, String> {
        let midi_out =
            MidiOutput::new("TraktorX1Legacy").map_err(|e| format!("MIDI output: {e}"))?;
        let midi_conn_out = midi_out
            .create_virtual(serial_number.as_str())
            .map_err(|e| format!("Create virtual MIDI output: {e}"))?;

        let board = X1mk1Board::from_yaml(&yaml_config)?;

        let mut led = [LED_DIM; 32];
        led[0] = 0x0c;
        led[31] = 0;

        Ok(Self {
            device,
            handle,
            serial_number,
            midi_conn_out,
            midi_conn_in: None,
            board,
            usb_buffer: [0; 24],
            usb_timeout: Duration::from_millis(50),
            usb_endpoint: Endpoint {
                address: USB_READ_FD,
                config: 1,
                interface: 0,
                setting: 0,
            },
            led,
            led_hotcue: [LED_DIM; 16],
            shift: 0,
            hotcue: false,
        })
    }

    fn init_midi_input(&mut self, sender: mpsc::Sender<Vec<u8>>) -> Result<(), String> {
        let midi_in = MidiInput::new("TraktorX1Legacy").map_err(|e| format!("MIDI input: {e}"))?;
        let midi_conn_in = midi_in
            .create_virtual(
                self.serial_number.as_str(),
                move |_stamp, message, _| {
                    let _ = sender.send(message.to_vec());
                },
                (),
            )
            .map_err(|e| format!("Create virtual MIDI input: {e}"))?;

        self.midi_conn_in = Some(midi_conn_in);
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        eprintln!("[bridge] starting {}", self.serial_number);

        let (midi_tx, midi_rx) = mpsc::channel::<Vec<u8>>();
        self.init_midi_input(midi_tx)?;
        self.configure_endpoint()
            .map_err(|e| format!("USB endpoint configuration: {e:?}"))?;
        self.update_leds()
            .map_err(|e| format!("Initial LED update: {e:?}"))?;

        loop {
            let mut led_changed = false;
            while let Ok(message) = midi_rx.try_recv() {
                led_changed |= self.process_midi_message(&message);
            }
            if led_changed {
                self.update_leds()
                    .map_err(|e| format!("LED feedback write: {e:?}"))?;
            }

            match self.handle.read_bulk(
                self.usb_endpoint.address,
                &mut self.usb_buffer,
                self.usb_timeout,
            ) {
                Ok(len) if len == self.usb_buffer.len() => {
                    self.read_state(self.usb_buffer);
                    self.update_leds()
                        .map_err(|e| format!("LED refresh: {e:?}"))?;
                }
                Ok(len) => {
                    eprintln!(
                        "[usb] ignoring partial state packet: {len}/{} bytes",
                        self.usb_buffer.len()
                    );
                }
                Err(rusb::Error::Timeout) => {
                    // The X1 can stop sending packets while idle. This is not a disconnect.
                }
                Err(e) => return Err(format!("USB read failed: {e:?}")),
            }
        }
    }

    fn process_midi_message(&mut self, message: &[u8]) -> bool {
        if message.len() < 3 {
            eprintln!("[midi] ignoring short message: {message:?}");
            return false;
        }

        let index = message[1] as usize;
        let value = message[2];

        match message[0] {
            MIDI_CHANNEL_LED if index < self.led.len() => {
                self.led[index] = value;
                true
            }
            MIDI_CHANNEL_HOTCUE if index < self.led_hotcue.len() => {
                self.led_hotcue[index] = if value != 0 { LED_BRIGHT } else { LED_DIM };
                true
            }
            MIDI_CHANNEL_HOTCUE => {
                eprintln!("[midi] invalid hotcue LED index: {index}");
                false
            }
            _ => false,
        }
    }

    fn configure_endpoint(&mut self) -> rusb::Result<()> {
        self.handle
            .set_active_configuration(self.usb_endpoint.config)?;
        self.handle.claim_interface(self.usb_endpoint.interface)?;
        self.handle
            .set_alternate_setting(self.usb_endpoint.interface, self.usb_endpoint.setting)?;
        Ok(())
    }

    fn read_state(&mut self, buf: [u8; 24]) {
        let mut binbyte = [[false; 8]; 5];

        hex2bool(buf[1], &mut binbyte[0]);
        hex2bool(buf[2], &mut binbyte[1]);
        hex2bool(buf[3], &mut binbyte[2]);
        hex2bool(buf[4], &mut binbyte[3]);
        hex2bool(buf[5], &mut binbyte[4]);

        if let Some(ButtonType::Hold(btn)) = self.board.buttons.get("SHIFT") {
            self.shift = binbyte[btn.read_i as usize][btn.read_j as usize] as u8;
        }

        for (ctrl_name, button_type) in &mut self.board.buttons {
            match button_type {
                ButtonType::Toggle(button) => {
                    if self.hotcue && button.hotcue_ignore {
                        continue;
                    }
                    button.curr = binbyte[button.read_i as usize][button.read_j as usize];
                    if button.curr != button.prev && button.curr {
                        let _ = self.midi_conn_out.send(&[
                            MIDI_CHANNEL + self.shift,
                            button.midi_ctrl_ch,
                            127,
                        ]);
                        if ctrl_name == "HOTCUE" {
                            self.hotcue = !self.hotcue;
                            self.led[button.write_idx as usize] =
                                if self.hotcue { LED_BRIGHT } else { LED_DIM };
                        }
                    }
                    button.prev = button.curr;
                }
                ButtonType::Hold(button) => {
                    if self.hotcue && button.hotcue_ignore {
                        continue;
                    }
                    button.curr = binbyte[button.read_i as usize][button.read_j as usize];
                    if button.curr == button.prev {
                        continue;
                    }

                    if button.curr {
                        button.shift_on_press = self.shift;
                        let channel = if ctrl_name == "SHIFT" {
                            MIDI_CHANNEL
                        } else {
                            MIDI_CHANNEL + self.shift
                        };
                        let _ = self
                            .midi_conn_out
                            .send(&[channel, button.midi_ctrl_ch, 127]);
                    } else {
                        let channel = if ctrl_name == "SHIFT" {
                            MIDI_CHANNEL
                        } else {
                            MIDI_CHANNEL + button.shift_on_press
                        };
                        let _ = self.midi_conn_out.send(&[channel, button.midi_ctrl_ch, 0]);
                    }

                    button.prev = button.curr;
                }
                ButtonType::Hotcue(button) => {
                    if !self.hotcue {
                        continue;
                    }
                    button.curr = binbyte[button.read_i as usize][button.read_j as usize];
                    if button.curr == button.prev {
                        continue;
                    }

                    if button.curr {
                        button.shift_on_press = self.shift;
                        let _ = self.midi_conn_out.send(&[
                            MIDI_CHANNEL_HOTCUE + self.shift,
                            button.midi_ctrl_ch,
                            127,
                        ]);
                    } else {
                        let _ = self.midi_conn_out.send(&[
                            MIDI_CHANNEL_HOTCUE + button.shift_on_press,
                            button.midi_ctrl_ch,
                            0,
                        ]);
                    }

                    button.prev = button.curr;
                }
                ButtonType::Knob(knob) => {
                    knob.curr = knob_to_midi(buf[knob.read_i as usize], buf[knob.read_j as usize]);
                    if knob.curr != knob.prev {
                        let _ = self.midi_conn_out.send(&[
                            MIDI_CHANNEL + self.shift,
                            knob.midi_ctrl_ch,
                            knob.curr,
                        ]);
                    }
                    knob.prev = knob.curr;
                }
                ButtonType::Encoder(encoder) => {
                    let mut binnum = [0; 8];
                    hex2bin(buf[encoder.read_i as usize], &mut binnum);
                    encoder.curr = match encoder.read_pos {
                        's' => binnum[0] + binnum[1] * 2 + binnum[2] * 4 + binnum[3] * 8,
                        'e' => binnum[4] + binnum[5] * 2 + binnum[6] * 4 + binnum[7] * 8,
                        other => {
                            eprintln!("[config] invalid encoder read_pos: {other}");
                            encoder.prev
                        }
                    };

                    if encoder.curr != encoder.prev {
                        let velocity = if encoder.prev == 15 && encoder.curr == 0 {
                            1
                        } else if encoder.prev == 0 && encoder.curr == 15 {
                            127
                        } else if encoder.curr < encoder.prev {
                            127
                        } else {
                            1
                        };

                        let _ = self.midi_conn_out.send(&[
                            MIDI_CHANNEL + self.shift,
                            encoder.midi_ctrl_ch,
                            velocity,
                        ]);
                    }
                    encoder.prev = encoder.curr;
                }
            }
        }
    }

    fn update_leds(&self) -> rusb::Result<()> {
        let mut led = self.led;

        if self.hotcue {
            led[9..25].copy_from_slice(&self.led_hotcue);
        }

        let written = self
            .handle
            .write_bulk(USB_WRITE_FD, &led, self.usb_timeout)?;
        if written != led.len() {
            return Err(rusb::Error::Other);
        }

        match self
            .handle
            .read_bulk(USB_UNLOCK_FD, &mut [0; 1], self.usb_timeout)
        {
            Ok(_) | Err(rusb::Error::Timeout) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
