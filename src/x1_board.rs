// Derived from Opa-/x1-mk1-usb2midi (Apache-2.0), modified for TraktorX1Legacy.
use std::collections::HashMap;

use crate::conf::{YamlButtonType, YamlConfig};

pub struct Button {
    pub curr: bool,
    pub prev: bool,
    pub read_i: u8,
    pub read_j: u8,
    pub write_idx: u8,
    pub midi_ctrl_ch: u8,
    pub hotcue_ignore: bool,
    pub shift_on_press: u8,
}

pub struct Knob {
    pub curr: u8,
    pub prev: u8,
    pub read_i: u8,
    pub read_j: u8,
    pub midi_ctrl_ch: u8,
}

pub struct Encoder {
    pub curr: u8,
    pub prev: u8,
    pub read_pos: char,
    pub read_i: u8,
    pub midi_ctrl_ch: u8,
}

pub enum ButtonType {
    Toggle(Button),
    Hold(Button),
    Knob(Knob),
    Encoder(Encoder),
    Hotcue(Button),
}

pub struct X1mk1Board {
    pub(crate) buttons: HashMap<String, ButtonType>,
}

impl X1mk1Board {
    pub(crate) fn from_yaml(yaml_config: &YamlConfig) -> Result<Self, String> {
        let mut buttons = HashMap::new();

        for yaml_button in &yaml_config.buttons {
            let require_read_j = || {
                yaml_button
                    .read_j
                    .ok_or_else(|| format!("{} requires read_j", yaml_button.name))
            };

            let button_type = match yaml_button.button_type {
                YamlButtonType::Toggle | YamlButtonType::Hold | YamlButtonType::Hotcue => {
                    let button = Button {
                        curr: false,
                        prev: false,
                        read_i: yaml_button.read_i,
                        read_j: require_read_j()?,
                        write_idx: yaml_button.write_idx.unwrap_or(0),
                        midi_ctrl_ch: yaml_button.midi_ctrl_ch,
                        hotcue_ignore: yaml_button.hotcue_ignore.unwrap_or(false),
                        shift_on_press: 0,
                    };
                    match yaml_button.button_type {
                        YamlButtonType::Toggle => ButtonType::Toggle(button),
                        YamlButtonType::Hold => ButtonType::Hold(button),
                        YamlButtonType::Hotcue => ButtonType::Hotcue(button),
                        _ => unreachable!(),
                    }
                }
                YamlButtonType::Knob => ButtonType::Knob(Knob {
                    curr: 0,
                    prev: 0,
                    read_i: yaml_button.read_i,
                    read_j: require_read_j()?,
                    midi_ctrl_ch: yaml_button.midi_ctrl_ch,
                }),
                YamlButtonType::Encoder => ButtonType::Encoder(Encoder {
                    curr: 0,
                    prev: 0,
                    read_pos: yaml_button
                        .read_pos
                        .ok_or_else(|| format!("{} requires read_pos", yaml_button.name))?,
                    read_i: yaml_button.read_i,
                    midi_ctrl_ch: yaml_button.midi_ctrl_ch,
                }),
            };

            buttons.insert(yaml_button.name.clone(), button_type);
        }

        Ok(Self { buttons })
    }
}
