use std::collections::HashMap;

use crossterm::{
    style::Color,
};
use serde_json::Value;

// pub type CommandFn = fn(&mut Engine) -> Result<(), String>;


pub struct Config {
    pub init_location: Option<String>,
    pub settings: HashMap<String, String>,
    pub theme: Theme,
}

impl Config {
    // pub fn get_style_color(&mut self, identifier: &str, default_val: Option<Color>) -> Color {
    //     match hex_to_color(self.styles[&identifier.to_string()].as_str()) {
    //         Ok(color) => color,
    //         Err(str) => {
    //             if let Some(def) = default_val {
    //                 def
    //             } else {
    //                 panic!(
    //                     "no style defined for colof {:?}, error: {:?}",
    //                     identifier, str
    //                 );
    //             }
    //         }
    //     }
    // }
}

// fn parse_keybinding(key_str: &str) -> Option<KeyEvent> {
//     let parts: Vec<&str> = key_str.split('-').collect();

//     let mut modifiers = KeyModifiers::empty();
//     let mut key_part = None;

//     for part in parts {
//         match part {
//             "C" | "Ctrl" => modifiers |= KeyModifiers::CONTROL,
//             "S" | "Shift" => modifiers |= KeyModifiers::SHIFT,
//             "A" | "Alt" => modifiers |= KeyModifiers::ALT,
//             _ => key_part = Some(part),
//         }
//     }

//     let key_part = key_part?;

//     let code = match key_part.to_lowercase().as_str() {
//         "enter" => KeyCode::Enter,
//         "tab" => KeyCode::Tab,
//         "esc" | "escape" => KeyCode::Esc,
//         "backspace" => KeyCode::Backspace,
//         "left" => KeyCode::Left,
//         "right" => KeyCode::Right,
//         "up" => KeyCode::Up,
//         "down" => KeyCode::Down,
//         c if c.len() == 1 => KeyCode::Char(c.chars().next()?),
//         _ => return None,
//     };

//     Some(KeyEvent {
//         code,
//         modifiers,
//         kind: KeyEventKind::Press,
//         state: KeyEventState::empty(),
//     })
// }

// pub fn parse_keymap(config: &HashMap<String, String>) -> HashMap<KeyEvent, String> {
//     let mut out = HashMap::new();

//     for (key_str, command) in config {
//         if let Some(key) = parse_keybinding(key_str) {
//             out.insert(key, command.clone());
//         } else {
//             eprintln!("Invalid keybinding: {}", key_str);
//         }
//     }

//     out
// }




pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');

    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string()).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string()).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string()).ok()?;

    Some(Color::Rgb { r, g, b })
}

pub fn blend(a: Color, b: Color, ratio: f32) -> Color {
    match (a, b) {
        (Color::Rgb { r: ar, g: ag, b: ab }, Color::Rgb { r: br, g: bg, b: bb }) => {
            let mix = |x, y| ((x as f32 * ratio) + (y as f32 * (1.0 - ratio))) as u8;
            Color::Rgb {
                r: mix(ar, br),
                g: mix(ag, bg),
                b: mix(ab, bb),
            }
        }
        _ => a, // fallback (shouldn't happen if everything is RGB)
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub background: Color,
    pub foreground: Color,

    pub selection_background: Color,
    pub selection_foreground: Color,
    pub url_color: Color,

    pub cursor: Color,
    pub cursor_text_color: Color,

    // tabs
    pub active_tab_background: Color,
    pub active_tab_foreground: Color,
    pub inactive_tab_background: Color,
    pub inactive_tab_foreground: Color,
    pub tab_bar_background: Color,

    // windows
    pub active_border_color: Color,
    pub inactive_border_color: Color,

    // palette
    pub colors: [Color; 18],
}

impl TryFrom<Value> for Theme {
    type Error = String;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let obj = value.as_object().ok_or_else(|| "theme must be an object".to_string())?;

        let get = |key: &str| -> Option<Color> {
            obj.get(key)
                .and_then(Value::as_str)
                .and_then(|s| hex_to_color(s))
        };

        // REQUIRED
        let background = get("background")
            .ok_or_else(|| "background is required".to_string())?;
        let foreground = get("foreground")
            .ok_or_else(|| "foreground is required".to_string())?;

        // DERIVED BASE COLORS
        let fg_dim = blend(foreground, background, 0.7);
        let fg_more_dim = blend(foreground, background, 0.5);
        let bg_lift = blend(background, foreground, 0.15);

        // SELECTION
        let selection_background =
            get("selection_background").unwrap_or(bg_lift);
        let selection_foreground =
            get("selection_foreground").unwrap_or(foreground);

        // CURSOR
        let cursor =
            get("cursor").unwrap_or(foreground);
        let cursor_text_color =
            get("cursor_text_color").unwrap_or(background);

        // URL
        let url_color =
            get("url_color").unwrap_or(blend(foreground, Color::Rgb { r: 0, g: 255, b: 255 }, 0.5));

        // TABS
        let active_tab_background =
            get("active_tab_background").unwrap_or(foreground);
        let active_tab_foreground =
            get("active_tab_foreground").unwrap_or(background);

        let inactive_tab_background =
            get("inactive_tab_background").unwrap_or(bg_lift);
        let inactive_tab_foreground =
            get("inactive_tab_foreground").unwrap_or(fg_dim);

        let tab_bar_background =
            get("tab_bar_background").unwrap_or(background);

        // WINDOWS
        let active_border_color =
            get("active_border_color").unwrap_or(active_tab_background);

        // inactive border ≈ 50% opacity → blend fg/bg evenly
        let inactive_border_color =
            get("inactive_border_color")
                .unwrap_or(blend(foreground, background, 0.5));

        // PALETTE (color0..color17)
        let mut colors = [foreground; 18];
        for i in 0..18 {
            let key = format!("color{}", i);
            colors[i] = get(&key).unwrap_or_else(|| {
                // sensible fallback:
                match i {
                    0 => background,
                    7 | 15 => foreground,
                    8 => fg_more_dim,
                    _ => fg_dim,
                }
            });
        }

        Ok(Theme {
            background,
            foreground,

            selection_background,
            selection_foreground,
            url_color,

            cursor,
            cursor_text_color,

            active_tab_background,
            active_tab_foreground,
            inactive_tab_background,
            inactive_tab_foreground,
            tab_bar_background,

            active_border_color,
            inactive_border_color,

            colors,
        })
    }
}