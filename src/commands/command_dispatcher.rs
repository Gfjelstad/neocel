use std::collections::HashMap;

use pyo3::prelude::*;
use serde_json::{Value, json};

use crate::{
    api::{API, APICaller},
    engine::{Engine, WindowState, document::DocType},
    input::input_engine::InputEngine,
    render::UI,
};

pub type CommandId = String;
#[derive(Clone, Debug)]
pub struct Command {
    pub id: CommandId,
    pub args: Vec<Value>,
}

pub struct CommandContext<'a> {
    pub fp: APICaller<'a>,
}
impl<'a> CommandContext<'a> {
    pub fn call(&mut self, id: String, params: Value) -> Option<Value> {
        println!("should actually quit");
        let res = (self.fp)(id, params);
        res.unwrap_or(None)
    }
}

type CommandResult = Result<Value, String>;
type CommandFn = dyn FnMut(&mut CommandContext, Vec<Value>) -> CommandResult;

pub struct CommandDispatcher {
    pub global: HashMap<String, CommandFunction>,
    pub per_document: HashMap<DocType, HashMap<String, CommandFunction>>,
    queue: CommandDispatchQueue,
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
            queue: vec![],
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

    pub fn dispatch(&mut self, doc_type: &DocType, cmd: &Command) -> Result<(), String> {
        // 1️⃣ Look for per-document override first
        if let Some(doc_cmds) = self.per_document.get_mut(&doc_type)
            && let Some(func) = doc_cmds.get(&cmd.id)
        {
            self.queue.push(CommandDispatchQueueItem::Doc(
                doc_type.clone(),
                cmd.id.clone(),
                cmd.args.clone(),
            ));
            // return Self::call_command_func(func, ctx, cmd.args.clone());
        }

        // 2️⃣ Fallback to global
        if self.global.contains_key(&cmd.id) {
            // return Self::call_command_func(func, ctx, cmd.args.clone());
            self.queue.push(CommandDispatchQueueItem::Global(
                cmd.id.clone(),
                cmd.args.clone(),
            ));
            return Ok(());
        }

        Err(format!("Command not found: {}", cmd.id))
    }
    // pub fn dispatch(
    //     &mut self,
    //     doc_type: &DocType,
    //     ctx: &mut CommandContext,
    //     cmd: &Command,
    // ) -> Result<(), String> {
    //     // 1️⃣ Look for per-document override first
    //     if let Some(doc_cmds) = self.per_document.get_mut(&doc_type)
    //         && let Some(func) = doc_cmds.get(&cmd.id)
    //     {
    //         self.queue.push(Box::new(*func.clone()));
    //         return Ok(());
    //         // return Self::call_command_func(func, ctx, cmd.args.clone());
    //     }
    //
    //     // 2️⃣ Fallback to global
    //     if let Some(func) = self.global.get(&cmd.id) {
    //         // return Self::call_command_func(func, ctx, cmd.args.clone());
    //         self.queue.push(Box::new(*func.clone()));
    //         return Ok(());
    //     }
    //
    //     Err(format!("Command not found: {}", cmd.id))
    // }

    fn call_command_func(
        func: &mut CommandFunction,
        ctx: &mut CommandContext,
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

    pub fn flush_queue(
        &mut self,
        engine: &mut Engine,
        input_engine: &mut InputEngine,
        ui: &mut UI,
        api: &mut API,
    ) -> Result<(), String> {
        let mut queue = Some(std::mem::take(&mut self.queue));
        let mut new_queue: CommandDispatchQueue = vec![];
        _ = api.run_command(engine, input_engine, ui, &mut new_queue, |caller| {
            let mut ctx = CommandContext { fp: caller };
            let queue = queue.take().expect("run_command called multiple times");
            for queued in queue {
                match queued {
                    CommandDispatchQueueItem::Global(id, args) => {
                        if let Some(func) = self.global.get_mut(&id) {
                            _ = Self::call_command_func(func, &mut ctx, args);
                        }
                    }
                    CommandDispatchQueueItem::Doc(doc_type, id, args) => {
                        if let Some(doc_fns) = self.per_document.get_mut(&doc_type)
                            && let Some(func) = doc_fns.get_mut(&id)
                        {
                            _ = Self::call_command_func(func, &mut ctx, args);
                        }
                    }
                    CommandDispatchQueueItem::RegisterGlobal(id, command_function) => {
                        self.global.insert(id, command_function);
                    }
                    CommandDispatchQueueItem::RegisterDoc(doc_type, id, command_function) => {
                        self.register_for_doc(doc_type, id.as_str(), command_function)
                    }
                }
            }
        });
        self.queue = new_queue;
        Ok(())
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
