pub mod command_api;
pub mod document_api;
pub mod engine_api;
pub mod text_document_api;
pub mod utils;
use std::collections::HashMap;

use pyo3::{Py, PyAny};
use serde_json::Value;

use crate::{
    commands::command_dispatcher::CommandDispatcher, engine::Engine,
    input::input_engine::InputEngine, render::UI,
};
#[derive(Debug)]
pub enum ExternalCommandInput {
    Python(Py<PyAny>),
    JSON(Value),
}

pub struct APIMethodParams<'a> {
    engine: &'a mut Engine,
    input_engine: &'a mut InputEngine,
    ui: &'a mut UI,
    command_dispatch: &'a mut CommandDispatcher,
    params: Option<ExternalCommandInput>,
}
pub type APIMethodResult = Result<Option<Value>, String>;
pub type APIMethod = for<'a, 'b> fn(&'b mut APIMethodParams<'a>) -> APIMethodResult;
pub struct API {
    commands: HashMap<String, APIMethod>,
}

pub type APICaller<'a> =
    &'a mut dyn FnMut(String, Option<ExternalCommandInput>) -> Result<Option<Value>, String>;

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
        let t: HashMap<String, APIMethod> = methods
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
        let mut callable = |command_name: String,
                            params: Option<ExternalCommandInput>|
         -> Result<Option<Value>, String> {
            if let Some(func) = self.commands.get(&command_name) {
                let mut tuple_args = APIMethodParams {
                    engine,
                    input_engine,
                    ui,
                    command_dispatch,
                    params,
                };
                func(&mut tuple_args)
            } else {
                println!("could not find command");
                Ok(None)
            }
        };

        // invoke the user-provided callback, passing in the callable
        callback(&mut callable)
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
