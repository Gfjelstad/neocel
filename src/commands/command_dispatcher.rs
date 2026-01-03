use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::{Value, json};

use crate::{
    api::APICaller,
    engine::{Engine, WindowState, document::DocType},
};

pub type CommandId = String;
#[derive(Clone, Debug)]
pub struct Command {
    pub id: CommandId,
    pub args: Vec<Value>,
}

pub struct CommandContext<'a> {
    pub api: APICaller<'a>,
}

type CommandResult = Result<Value, String>;
type CommandFn = dyn FnMut(CommandContext, Vec<Value>) -> CommandResult;

pub struct CommandDispatcher {
    pub global: HashMap<String, CommandFunction>,
    pub per_document: HashMap<DocType, HashMap<String, CommandFunction>>,
}
impl CommandDispatcher {
    pub fn new() -> Self {
        Self {
            global: HashMap::new(),
            per_document: HashMap::new(),
        }
    }
    pub fn register_global(&mut self, id: &str, func: CommandFunction) {
        self.global.insert(id.to_string(), func);
    }

    pub fn register_for_doc(&mut self, doc_type: DocType, id: &str, func: CommandFunction) {
        self.per_document
            .entry(doc_type)
            .or_default()
            .insert(id.to_string(), func);
    }
    pub fn dispatch(
        &mut self,
        doc_type: &DocType,
        ctx: CommandContext,
        cmd: &Command,
    ) -> CommandResult {
        // 1️⃣ Look for per-document override first
        if let Some(doc_cmds) = self.per_document.get_mut(&doc_type)
            && let Some(func) = doc_cmds.get_mut(&cmd.id)
        {
            return Self::call_command_func(func, ctx, cmd.args.clone());
        }

        // 2️⃣ Fallback to global
        if let Some(func) = self.global.get_mut(&cmd.id) {
            return Self::call_command_func(func, ctx, cmd.args.clone());
        }

        Err(format!("Command not found: {}", cmd.id))
    }

    fn call_command_func(
        func: &mut CommandFunction,
        mut ctx: CommandContext,
        args: Vec<Value>,
    ) -> CommandResult {
        match func {
            CommandFunction::Rust(f) => f(ctx, args),
            _ => Ok(json!({"error": "python not implimented",})), // CommandFunction::Python(py_fn) => py_fn
                                                                  //     .call1(py, (ctx, args))
                                                                  //     .map(|v| v.extract::<Value>(py).unwrap_or(Value::Null))
                                                                  //     .map_err(|e| e.to_string()),
        }
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}
pub enum CommandFunction {
    Rust(Box<CommandFn>),
    Python(Py<PyAny>),
}
