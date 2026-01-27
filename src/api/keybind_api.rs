use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    vec,
};

use pyo3::{
    Py, PyAny, PyErr, Python,
    exceptions::{PyRuntimeError, PyValueError},
    pyclass, pymethods,
    types::{PyAnyMethods, PyDict},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    api::PyValue,
    commands::command_dispatcher::{CommandDispatcher, CommandFunction, CommandRequest},
    engine::{Engine, document::DocType},
    input::{
        Token, input_engine::InputEngine, keymaps::ActionNode
    },
};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct KeybindApi {
    engine: Arc<Mutex<Engine>>,
    command_dispatcher: Arc<Mutex<CommandDispatcher>>,
    input_engine: Arc<Mutex<InputEngine>>
}

impl KeybindApi {
    pub fn new(
        engine: Arc<Mutex<Engine>>,
        command_dispatcher: Arc<Mutex<CommandDispatcher>>,
        input_engine: Arc<Mutex<InputEngine>>
    ) -> Self {
        Self {
            engine,
            command_dispatcher,
            input_engine
        }
    }
}

#[pymethods]
impl KeybindApi {
    #[pyo3(signature = (mode, keybind, function=None, command=None, options = None))]
    pub fn bind(
        &self,
        mode: char,
        keybind: Vec<String>,
        function: Option<Py<PyAny>>,
        command: Option<PyValue>,
        options: Option<PyValue>,
    ) -> Result<(), PyErr> {
        if (function.is_none() && command.is_none()) || (function.is_some() && command.is_some()) {
            return Err(PyValueError::new_err(
                "either command or function is required",
            ));
        }
        let options = if let Some(o) = options {o.parse_to::<BindOptions>()?} else {BindOptions {doc: None, doc_type: None}};

        let command = if let Some(func) = function {
            let temp_id = uuid::Uuid::new_v4().to_string();
            self.command_dispatcher
                .lock()
                .unwrap()
                .register_global(temp_id.as_str(), CommandFunction::Python(func));
            CommandRequest {
                id: temp_id,
                args: vec![],
            }
        } else if let Some(command) = command {
            command.parse_to::<CommandRequest>()?
        } else {
            return Err(PyValueError::new_err(
                "either command or function is required",
            ));
        };

        let mut guarded_input_engine = self.input_engine.lock().unwrap();
        let curr_mode = guarded_input_engine.mode();
        if let Some(doc_id) = options.doc  {
           let doc_binds = guarded_input_engine.document_map.entry(doc_id).or_insert_with(HashMap::new);
            
        }

        // let mut engine_guard = self.engine.lock().unwrap();

        // let provider: &mut dyn KeymapProvider = if let Some(Some(doc_id)) = options.map(|d| d.doc) {
        //     let doc = engine_guard.docs.get_mut(doc_id.as_str());
        //     if let Some(doc) = doc {
        //         doc as &mut dyn KeymapProvider
        //     } else {
        //         return Err(PyRuntimeError::new_err(format!(
        //             "document not found for {}",
        //             doc_id
        //         )));
        //     }
        // } else {
        //     &mut *engine_guard as &mut dyn KeymapProvider
        // };
        // let mut ref_map = provider.keymap().clone();

        // insert_into_tree(&mut ref_map, &keybind, Token::Command(command))
        //     .map_err(|e| PyRuntimeError::new_err(e))?;

        // provider.set_keymap_cache(Some(ref_map));

        // let mut vecoutput = vec![];
        // provider.keymap().collect(&mut vecoutput);
        // for item in vecoutput {
        //     match item {
        //         Token::Command(val) => {
        //             println!("{}", val.id);
        //         }
        //         _=>{}
        //     }
        // }

        Ok(())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CommandParams {
    pub id: String,
    pub params: Vec<Value>,
}


#[derive(Debug, Clone, Deserialize)]
pub struct BindOptions {
    doc: Option<String>,
    doc_type: Option<DocType>
}
