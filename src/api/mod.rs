use std::collections::HashMap;

use serde_json::Value;

use crate::{
    commands::command_dispatcher::CommandDispatcher, engine::Engine,
    input::input_engine::InputEngine, render::UI,
};

pub type APIMethodParams<'a> = (
    &'a mut Engine,
    &'a mut InputEngine,
    &'a mut UI,
    &'a mut CommandDispatcher,
    Value,
);
pub type APIMethod = fn(&mut APIMethodParams) -> Result<(), String>;
pub struct API {
    commands: HashMap<String, APIMethod>,
    queue: Vec<(String, Value)>,
}

impl API {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            queue: vec![],
        }
    }
    pub fn queue(&mut self, method: &str, params: Value) {
        self.queue.push((method.to_string(), params));
    }

    pub fn run(state: APIMethodParams) -> Result<(), String> {
        Ok(())
    }
}

impl Default for API {
    fn default() -> Self {
        Self::new()
    }
}
