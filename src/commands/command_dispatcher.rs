use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::{Value, json};

use crate::engine::{Engine, WindowState, document::DocType};

pub type CommandId = String;
pub struct Command {
    id: CommandId,
    args: Vec<Value>,
}

pub struct CommandContext<'a> {
    pub engine: &'a mut Engine,
    pub window_state: &'a mut WindowState,
}

type CommandResult = Result<Value, String>;
type CommandFn = dyn FnMut(CommandContext, Vec<Value>) -> CommandResult;

pub struct CommandDispatcher {
    pub global: HashMap<String, CommandFunction>,
    pub per_document: HashMap<DocType, HashMap<String, CommandFunction>>,
}
impl CommandDispatcher {
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
        doc_type: DocType,
        ctx: CommandContext,
        cmd: Command,
        py: Python,
    ) -> CommandResult {
        // 1️⃣ Look for per-document override first
        if let Some(doc_cmds) = self.per_document.get_mut(&doc_type) {
            if let Some(func) = doc_cmds.get_mut(&cmd.id) {
                return Self::call_command_func(func, ctx, cmd.args.clone(), py);
            }
        }

        // 2️⃣ Fallback to global
        if let Some(func) = self.global.get_mut(&cmd.id) {
            return Self::call_command_func(func, ctx, cmd.args.clone(), py);
        }

        Err(format!("Command not found: {}", cmd.id))
    }

    fn call_command_func(
        func: &mut CommandFunction,
        mut ctx: CommandContext,
        args: Vec<Value>,
        py: Python,
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
pub enum CommandFunction {
    Rust(Box<CommandFn>),
    Python(Py<PyAny>),
}
