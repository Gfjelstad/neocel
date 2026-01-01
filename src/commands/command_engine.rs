use serde_json::Value;

use crate::engine::Engine;

pub enum Key {
    Char(char),
    Ctrl(char),
    Alt(char),
    Esc,
    Enter,
    Backspace,
}

pub struct Command {
    pub id: String,
    pub args: Vec<Value>,
}
pub struct CommandEngine {}

impl CommandEngine {
    pub fn feed(key: Key, engine: &Engine) -> Option<Command> {
        None
    }
}
