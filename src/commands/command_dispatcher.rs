use crate::{
    api::{API, APICaller, ExternalCommandInput},
    engine::{Engine, document::DocType},
    input::input_engine::InputEngine,
    render::UI,
};
use pyo3::{ffi::PyCFunction, prelude::*, types::PyDict};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
        cmd: &CommandRequest,
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
            CommandFunction::Internal(id, params) => ctx.call(
                id.clone(),
                Some(ExternalCommandInput::JSON(params.clone().unwrap())),
            ),
            CommandFunction::Python(py_func) => {
                Python::with_gil(|py| {
                    let py_args = pythonize::pythonize(py, &args)
                        .map_err(|e| format!("Failed to convert args: {}", e))?;
                    let pyapi = ctx.to_py_api()?;
                    // Create API context with raw pointer
                    let result = py_func
                        .call1(py, (pyapi, py_args))
                        .map_err(|e| format!("Python call failed: {}", e))?;

                    if result.is_none(py) {
                        Ok(None)
                    } else {
                        pythonize::depythonize(result.bind(py))
                            .map(Some)
                            .map_err(|e| format!("Failed to deserialize result: {}", e))
                    }
                })
            }
        }
    }
}

impl Default for CommandDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

pub type CommandId = String;
#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct CommandRequest {
    pub id: CommandId,
    pub args: Vec<Value>,
}

pub struct CommandContext<'a> {
    fp: APICaller<'a>,
}
impl<'a> CommandContext<'a> {
    pub fn call(
        &mut self,
        id: String,
        params: Option<ExternalCommandInput>,
    ) -> Result<Option<Value>, String> {
        (self.fp)(id, params)
    }
    pub fn to_py_api(&mut self) -> Result<Py<ApiContext>, String> {
        Python::attach(|py| {
            let fp_ptr = self.fp
                as *mut dyn FnMut(
                    String,
                    Option<ExternalCommandInput>,
                ) -> Result<Option<Value>, String>;
            let static_ptr: *mut (
                dyn FnMut(String, Option<ExternalCommandInput>) -> Result<Option<Value>, String>
                    + 'static
            ) = unsafe { std::mem::transmute(fp_ptr) };

            Py::new(py, ApiContext { fp_ptr: static_ptr }).map_err(|e| e.to_string())
        })
    }
}

type CommandResult = Result<Option<Value>, String>;
type CommandFn = dyn FnMut(&mut CommandContext, Vec<Value>) -> CommandResult;

pub struct CommandDispatcher {
    pub global: HashMap<String, CommandHandle>,
    pub per_document: HashMap<DocType, HashMap<String, CommandHandle>>,
}
pub type CommandHandle = Rc<RefCell<CommandFunction>>;
pub enum CommandFunction {
    Rust(Box<CommandFn>),
    Python(Py<PyAny>),
    Internal(String, Option<Value>),
}
#[pyclass(unsendable)]
pub struct ApiContext {
    fp_ptr: *mut (
        dyn FnMut(String, Option<ExternalCommandInput>) -> Result<Option<Value>, String> + 'static
    ),
}

#[pymethods]
impl ApiContext {
    fn call(&self, id: String, params: Option<Py<PyAny>>) -> PyResult<Py<PyAny>> {
        Python::attach(|py| {
            let input = params.map(|p| ExternalCommandInput::Python(p));

            // UNSAFE: Call through the raw pointer
            let result = unsafe { (*self.fp_ptr)(id, input) }
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;

            let res = match result {
                Some(value) => pythonize::pythonize(py, &value)
                    .map(|v| v.into_pyobject(py).unwrap().unbind())
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string())),
                None => Ok(py.None()),
            };
            res
        })
    }
}
