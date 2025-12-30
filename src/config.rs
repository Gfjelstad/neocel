use std::collections::HashMap;

use crate::engine::Engine;

pub struct Config {
    pub keybinds: HashMap<String, String>,
    pub settings: HashMap<String, String>,
    pub styles: HashMap<String, String>,
    pub commands: HashMap<String, Box<dyn FnMut(&mut Engine)>>,
}
