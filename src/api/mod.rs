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
    &'a mut Value,
);
pub type APIMethod = fn(&mut APIMethodParams) -> Result<Option<Value>, String>;
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

    pub fn register_api(&mut self, methods: HashMap<&str, APIMethod>) {
        let transformed: HashMap<String, APIMethod> = methods
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self.commands.extend(transformed);
    }

    pub fn with_api<F>(
        &mut self,
        engine: &mut Engine,
        input_engine: &mut InputEngine,
        ui: &mut UI,
        command_dispatch: &mut CommandDispatcher,
        mut callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(&mut dyn FnOnce(String, Value) -> Result<Option<Value>, String>),
    {
        // define the callable function that executes commands
        let mut callable =
            |command_name: String, mut params: Value| -> Result<Option<Value>, String> {
                if let Some(func) = self.commands.get(&command_name) {
                    // assuming func expects a mutable tuple of references
                    let mut tuple_args = (engine, input_engine, ui, command_dispatch, &mut params);
                    func(&mut tuple_args)
                } else {
                    Ok(None)
                }
            };

        // invoke the user-provided callback, passing in the callable
        callback(&mut callable);

        Ok(())
    }
}

impl Default for API {
    fn default() -> Self {
        Self::new()
    }
}
