use std::collections::HashMap;

use crate::input::{Token, input_engine::Mode};

use crossterm::event::{KeyCode as CtKey, KeyEvent, KeyModifiers};

use bitflags::bitflags;
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct ActionNode {
    pub children: HashMap<Key, ActionNode>,
    pub action: Option<Token>,
}
impl ActionNode {
    pub fn new() -> Self {
        Self {
            action: None,
            children: HashMap::new(),
        }
    }

    pub fn collect(&self, out: &mut Vec<Token>) {
        if let Some(token) = &self.action {
            out.push(token.clone());
        }

        for child in self.children.values() {
            child.collect(out);
        }
    }

   
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const CTRL  = 0b0001;
        const ALT   = 0b0010;
        const SHIFT = 0b0100;
        const SUPER = 0b1000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    Char(char),
    Enter,
    Esc,
    Backspace,
    Tab,
    BackTab,
    F(u8),
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub code: KeyCode,
    pub modifiers: Modifiers,
}

impl Key {
    pub fn is_printable(&mut self) -> bool {
        match self.code {
            KeyCode::Char(c) => {
                // Optional: skip control characters
                !c.is_control()
            }
            KeyCode::Tab => true,     // tabs are printable
            KeyCode::BackTab => true, // shift-tab
            KeyCode::Enter => false,  // usually special
            KeyCode::Esc => false,
            KeyCode::Backspace => false,
            _ => false,
        }
    }
}

impl From<KeyEvent> for Key {
    fn from(ev: KeyEvent) -> Self {
        let mut mods = Modifiers::empty();

        if ev.modifiers.contains(KeyModifiers::CONTROL) {
            mods |= Modifiers::CTRL;
        }
        if ev.modifiers.contains(KeyModifiers::ALT) {
            mods |= Modifiers::ALT;
        }
        if ev.modifiers.contains(KeyModifiers::SHIFT) {
            mods |= Modifiers::SHIFT;
        }

        let code = match ev.code {
            CtKey::Char(c) => KeyCode::Char(c),
            CtKey::Enter => KeyCode::Enter,
            CtKey::BackTab => KeyCode::BackTab,
            CtKey::Esc => KeyCode::Esc,
            CtKey::Backspace => KeyCode::Backspace,
            CtKey::Up => KeyCode::Up,
            CtKey::Down => KeyCode::Down,
            CtKey::Left => KeyCode::Left,
            CtKey::Right => KeyCode::Right,
            _ => KeyCode::Esc,
        };

        Key {
            code,
            modifiers: mods,
        }
    }
}

impl TryFrom<&str> for Key {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('+').collect();
        let mut modifiers = Modifiers::empty();
        let mut key_part = value;

        if parts.len() > 1 {
            key_part = parts.last().unwrap();
            for modifier in &parts[..parts.len() - 1] {
                match modifier.to_lowercase().as_str() {
                    "ctrl" | "control" => modifiers |= Modifiers::CTRL,
                    "alt" => modifiers |= Modifiers::ALT,
                    "shift" => modifiers |= Modifiers::SHIFT,
                    "super" | "meta" | "cmd" => modifiers |= Modifiers::SUPER,
                    _ => return Err(format!("Unknown modifier: {}", modifier)),
                }
            }
        }

        let code = match key_part.to_lowercase().as_str() {
            "enter" | "return" => KeyCode::Enter,
            "esc" | "escape" => KeyCode::Esc,
            "backspace" => KeyCode::Backspace,
            "tab" => KeyCode::Tab,
            "backtab" => KeyCode::BackTab,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            "space" => KeyCode::Char(' '),
            s if s.starts_with('f') && s.len() > 1 => {
                let num = s[1..]
                    .parse::<u8>()
                    .map_err(|_| format!("Invalid function key: {}", s))?;
                KeyCode::F(num)
            }
            s if s.len() == 1 => KeyCode::Char(s.chars().next().unwrap()),
            _ => return Err(format!("Unknown key: {}", key_part)),
        };

        Ok(Key { code, modifiers })
    }
}

trait Bindable {
    fn bind(&mut self, mode: Mode, key_sequence: &[String], action: Token) ->  Result<(), String>;
}

impl Bindable for HashMap<Mode, ActionNode> {
    fn bind(&mut self, mode: Mode, key_sequence: &[String], action: Token) ->  Result<(), String> {

        let mut current = self.entry(mode)
            .or_insert_with(ActionNode::new);

        for (i, key_str) in key_sequence.iter().enumerate() {
            let key = Key::try_from(key_str.as_str())?;

            if i == key_sequence.len() - 1 {
                let node = current.children.entry(key).or_insert_with(ActionNode::new);
                node.action = Some(action.clone());
            } else {
                current = current.children.entry(key).or_insert_with(ActionNode::new);
            }
        }

    Ok(())
    }
}

// fn parse_key(key_str: &str) -> Result<Key, String> {

// }

pub fn build_keymap_tree(key_sequence: &Vec<String>, action: Token) -> Result<ActionNode, String> {
    let mut root = ActionNode::new();
    let mut current = &mut root;

    for (i, key_str) in key_sequence.iter().enumerate() {
        let key = Key::try_from(key_str.as_str())?;

        if i == key_sequence.len() - 1 {
            // Last key in sequence - set the action
            let node = current.children.entry(key).or_insert_with(ActionNode::new);
            node.action = Some(action.clone());
        } else {
            // Intermediate key - just create the node
            current = current.children.entry(key).or_insert_with(ActionNode::new);
        }
    }

    Ok(root)
}

pub fn insert_into_tree(
    root: &mut ActionNode,
    key_sequence: &[String],
    action: Token,
) -> Result<(), String> {
    let mut current = root;

    for (i, key_str) in key_sequence.iter().enumerate() {
        let key = Key::try_from(key_str.as_str())?;

        if i == key_sequence.len() - 1 {
            let node = current.children.entry(key).or_insert_with(ActionNode::new);
            node.action = Some(action.clone());
        } else {
            current = current.children.entry(key).or_insert_with(ActionNode::new);
        }
    }

    Ok(())
}
