pub mod command_dispatcher;
pub mod globals;

use crossterm::event::{KeyCode as CtKey, KeyEvent, KeyModifiers};

use bitflags::bitflags;

use crate::commands::command_dispatcher::CommandDispatcher;
pub trait CommandRegistry {
    fn register_commands(dispatcher: &mut CommandDispatcher) -> Result<(), String>;
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
