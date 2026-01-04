pub mod engine_api;
pub mod utils;
use std::collections::HashMap;

use serde_json::{Value, to_string};

use crate::{
    commands::command_dispatcher::{CommandDispatchQueue, CommandDispatcher},
    engine::Engine,
    input::input_engine::InputEngine,
    render::UI,
};

pub struct APIMethodParams<'a> {
    engine: &'a mut Engine,
    input_engine: &'a mut InputEngine,
    ui: &'a mut UI,
    command_dispatch: &'a mut CommandDispatchQueue,
    params: Value,
}
pub type APIMethodResult = Result<Option<Value>, String>;
pub type APIMethod = for<'a, 'b> fn(&'b mut APIMethodParams<'a>) -> APIMethodResult;
pub struct API {
    commands: HashMap<String, APIMethod>,
}

pub type APICaller<'a> = &'a mut dyn FnMut(String, Value) -> Result<Option<Value>, String>;

impl API {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }
    pub fn register_api(&mut self, methods: HashMap<&str, APIMethod>) {
        let mut t: HashMap<String, APIMethod> = methods
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        self.commands.extend(t);
    }

    pub fn run_command<F>(
        &mut self,
        engine: &mut Engine,
        input_engine: &mut InputEngine,
        ui: &mut UI,
        command_dispatch: &mut CommandDispatchQueue,
        mut callback: F,
    ) -> Result<(), String>
    where
        F: FnMut(APICaller),
    {
        // define the callable function that executes commands
        let mut callable =
            |command_name: String, mut params: Value| -> Result<Option<Value>, String> {
                if let Some(func) = self.commands.get(&command_name) {
                    // assuming func expects a mutable tuple of references
                    let mut tuple_args = APIMethodParams {
                        engine,
                        input_engine,
                        ui,
                        command_dispatch,
                        params: params,
                    };
                    func(&mut tuple_args)
                } else {
                    println!("could not find command");
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

pub trait APIRegister {
    fn register_methods(api: &mut API);
}
