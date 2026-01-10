use std::{cell::RefCell, collections::HashMap, rc::Rc};

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::{
    api::{API, APICaller},
    engine::{Engine, document::DocType},
    input::input_engine::InputEngine,
    render::UI,
};

pub type CommandId = String;
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Command {
    pub id: CommandId,
    pub args: Vec<Value>,
}

pub struct CommandContext<'a> {
    fp: APICaller<'a>,
}
impl<'a> CommandContext<'a> {
    pub fn call(&mut self, id: String, params: Option<Value>) -> Result<Option<Value>, String> {
        (self.fp)(id, params)
    }
}

type CommandResult = Result<Option<Value>, String>;
type CommandFn = dyn FnMut(&mut CommandContext, Vec<Value>) -> CommandResult;

pub struct CommandDispatcher {
    pub global: HashMap<String, CommandHandle>,
    pub per_document: HashMap<DocType, HashMap<String, CommandHandle>>,
}
pub type CommandDispatchQueue = Vec<CommandDispatchQueueItem>;
pub enum CommandDispatchQueueItem {
    Global(String, Vec<Value>),
    Doc(DocType, String, Vec<Value>),
    RegisterGlobal(String, CommandFunction),
    RegisterDoc(DocType, String, CommandFunction),
}
impl CommandDispatcher {
    pub fn new() -> Self {
        Self {
            global: HashMap::new(),
            per_document: HashMap::new(),
        }
    }
    pub fn register_global(&mut self, id: &str, func: CommandFunction) {
        self.global
            .insert(id.to_string(), Rc::new(RefCell::new(func)));
    }

    pub fn register_for_doc(&mut self, doc_type: DocType, id: &str, func: CommandFunction) {
        self.per_document
            .entry(doc_type)
            .or_default()
            .insert(id.to_string(), Rc::new(RefCell::new(func)));
    }

    pub fn dispatch(
        &mut self,
        cmd: &Command,
        engine: &mut Engine,
        input_engine: &mut InputEngine,
        ui: &mut UI,
    ) -> Result<Option<Value>, String> {
        let doc_type = &engine.get_current_window().1.doc_type.clone();
        let selected_command = self
            .per_document
            .get(doc_type)
            .and_then(|m| m.get(&cmd.id))
            .cloned()
            .or_else(|| self.global.get(&cmd.id).cloned())
            .ok_or_else(|| format!("Command not found: {}", cmd.id))?;

        let mut api = API::new();

        api.run_command(engine, input_engine, ui, self, move |caller| {
            let mut ctx = CommandContext { fp: caller };
            let mut cmd_fn = selected_command.borrow_mut();

            Self::call_command_func(&mut cmd_fn, &mut ctx, cmd.args.clone())
        })
    }

    fn call_command_func(
        func: &mut CommandFunction,
        ctx: &mut CommandContext,
        args: Vec<Value>,
    ) -> CommandResult {
        match func {
            CommandFunction::Rust(f) => f(ctx, args),
            CommandFunction::Internal(id, params) => ctx.call(id.clone(), params.clone()),
            CommandFunction::Python(_) => Ok(Some(json!({ "error": "python not implemented" }))),
        }
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
pub type CommandHandle = Rc<RefCell<CommandFunction>>;
pub enum CommandFunction {
    Rust(Box<CommandFn>),
    Python(Py<PyAny>),
    Internal(String, Option<Value>),
}
