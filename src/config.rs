use std::collections::HashMap;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    style::Color,
};

use crate::{engine::Engine, render::styling::hex_to_color};
pub type CommandFn = fn(&mut Engine) -> Result<(), String>;
pub struct Config {
    pub init_location: Option<String>,
    pub keybinds: HashMap<KeyEvent, String>,
    pub settings: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub commands: HashMap<String, CommandFn>,
}

impl Config {
    pub fn get_style_color(&mut self, identifier: &str, default_val: Option<Color>) -> Color {
        match hex_to_color(self.styles[&identifier.to_string()].as_str()) {
            Ok(color) => color,
            Err(str) => {
                if let Some(def) = default_val {
                    def
                } else {
                    panic!(
                        "no style defined for colof {:?}, error: {:?}",
                        identifier, str
                    );
                }
            }
        }
    }
}

fn parse_keybinding(key_str: &str) -> Option<KeyEvent> {
    let parts: Vec<&str> = key_str.split('-').collect();

    let mut modifiers = KeyModifiers::empty();
    let mut key_part = None;

    for part in parts {
        match part {
            "C" | "Ctrl" => modifiers |= KeyModifiers::CONTROL,
            "S" | "Shift" => modifiers |= KeyModifiers::SHIFT,
            "A" | "Alt" => modifiers |= KeyModifiers::ALT,
            _ => key_part = Some(part),
        }
    }

    let key_part = key_part?;

    let code = match key_part.to_lowercase().as_str() {
        "enter" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "esc" | "escape" => KeyCode::Esc,
        "backspace" => KeyCode::Backspace,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        c if c.len() == 1 => KeyCode::Char(c.chars().next()?),
        _ => return None,
    };

    Some(KeyEvent {
        code,
        modifiers,
        kind: KeyEventKind::Press,
        state: KeyEventState::empty(),
    })
}

pub fn parse_keymap(config: &HashMap<String, String>) -> HashMap<KeyEvent, String> {
    let mut out = HashMap::new();

    for (key_str, command) in config {
        if let Some(key) = parse_keybinding(key_str) {
            out.insert(key, command.clone());
        } else {
            eprintln!("Invalid keybinding: {}", key_str);
        }
    }

    out
}
