pub mod command_api;
pub mod document_api;
pub mod engine_api;
pub mod text_document_api;
pub mod utils;
use std::collections::HashMap;

use serde_json::{Value};

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
    command_dispatch: &'a mut CommandDispatcher,
    params: Option<Value>,
}
pub type APIMethodResult = Result<Option<Value>, String>;
pub type APIMethod = for<'a, 'b> fn(&'b mut APIMethodParams<'a>) -> APIMethodResult;
pub struct API {
    commands: HashMap<String, APIMethod>,
}

pub type APICaller<'a> = &'a mut dyn FnMut(String, Option<Value>) -> Result<Option<Value>, String>;

impl API {
    pub fn new() -> Self {
        let mut s = Self {
            commands: HashMap::new(),
        };
        engine_api::EngineAPI::register_methods(&mut s);
        command_api::CommandAPI::register_methods(&mut s);
        document_api::DocumentAPI::register_methods(&mut s);
        text_document_api::TextDocumentAPI::register_methods(&mut s);
        s
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
        command_dispatch: &mut CommandDispatcher,
        mut callback: F,
    ) -> Result<Option<Value>, String>
    where
        F: FnMut(APICaller) -> Result<Option<Value>, String>,
    {
        // define the callable function that executes commands
        let mut callable =
            |command_name: String, mut params: Option<Value>| -> Result<Option<Value>, String> {
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
        return callback(&mut callable);
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
