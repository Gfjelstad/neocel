use std::{collections::HashMap, rc::Rc};

use serde_json::Value;

use crate::{
    commands::{Key, command_dispatcher::Command},
    engine::Engine,
    input::Token,
};

pub struct TokenNode<T> {
    pub children: HashMap<Key, TokenNode<T>>,
    pub action: Option<T>,
}

pub enum Mode {
    Input,
    Visualize,
    Normal,
}
pub struct PendingState {
    pub count: u16,
    pub operator: Option<String>,
    modifier: Option<String>,
}
pub struct InputEngine {
    pub mode: Mode,
    pending: PendingState,
}

impl InputEngine {
    pub fn feed(key: Key, engine: &Engine) -> Option<Command> {
        None
    }
}
