use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use pyo3::{
    IntoPyObject, Py, PyAny, PyErr, Python,
    exceptions::PyRuntimeError,
    pyclass, pymethods,
    types::{PyAnyMethods, PyFunction},
};
use serde::Deserialize;
use serde_json::Value;

use crate::{api::PyValue, commands::{
        command_dispatcher::{CommandDispatcher, CommandFunction, CommandRequest},
        insert_into_tree,
    }, engine::{Engine, document::DocType}, input::{
        Token,
        input_engine::InputEngine,
        keymaps::{ActionNode, KeymapProvider},
    }
};
#[pyclass(unsendable)]
#[derive(Clone)]
pub struct CommandAPI {
    command_dispatcher: Arc<Mutex<CommandDispatcher>>,
    input_engine: Arc<Mutex<InputEngine>>,
    engine: Arc<Mutex<Engine>>,
}

impl CommandAPI {
    pub fn new(
        command_dispatcher: Arc<Mutex<CommandDispatcher>>,
        input_engine: Arc<Mutex<InputEngine>>,
        engine: Arc<Mutex<Engine>>,
    ) -> Self {
        Self {
            command_dispatcher,
            input_engine,
            engine,
        }
    }
}

// pub struct CommandAPI {}

#[pymethods]
impl CommandAPI {
    #[pyo3(signature = (id, params=None))]
    pub fn run(&self, id: String, params: Option<Vec<PyValue>>) -> Result<Option<PyValue>, PyErr> {
        let params = params.map(|vec| vec.into_iter().map(|pv| pv.0).collect());
        let result = CommandDispatcher::dispatch(
            &CommandRequest {
                id,
                args: params.unwrap_or_default(),
            },
            self.command_dispatcher.clone(),
            self.engine.clone(),
            self.input_engine.clone(),
        )
        .map_err(PyRuntimeError::new_err)?;

        Ok(match result {
            Some(val) => Some(PyValue(val)),
            None => None
        })
    }

    pub fn list_current(&self) -> Result<Vec<String>, PyErr> {
        let mut tokens: Vec<Token> = vec![];

        let _ = self.engine
            .lock()
            .unwrap()
            .keymap().children.values()
            .map(|node| {
                node.collect(&mut tokens);
            });

        let mut result : Vec<String> = vec![];
        
        for token in tokens {
            match token {
                Token::Operator(op) => {
                    result.push(op);
                }
                Token::Command(cmd) => {
                    result.push(cmd.id);
                },
                _ => {}
            }
        }


        Ok(result)
    }

    #[pyo3(signature = (id, function, doctype=None))]
    pub fn register(
        &self,
        id: String,
        function: Py<PyAny>,
        doctype: Option<DocType>,
    ) -> Result<(), PyErr> {
        let mut guarded_dispatcher = self.command_dispatcher.lock().unwrap();
        match doctype {
            Some(doc_type) => {
                guarded_dispatcher.register_for_doc(
                    doc_type,
                    id.as_str(),
                    CommandFunction::Python(function),
                );
            }
            None => {
                guarded_dispatcher.register_global(id.as_str(), CommandFunction::Python(function));
            }
        }

        Ok(())
    }

    // pub fn register_keybind(state: &mut APIMethodParams) -> APIMethodResult {
    //     let command = try_parse::<RegisterKeybind>(&state.params)?;

    //     // let tree = build_keymap_tree(&command.keys, Token::Command(CommandRequest { id: command.command_id.unwrap().clone(), args: command.params }))?;

    //     let mut map = state.engine.keymap().clone();

    //     if command.command_id.is_some() {
    //         insert_into_tree(
    //             &mut map,
    //             &command.keys,
    //             Token::Command({
    //                 CommandRequest {
    //                     id: command.command_id.unwrap().clone(),
    //                     args: command.params.unwrap_or_default(),
    //                 }
    //             }),
    //         )?;
    //     }

    //     state.engine.set_keymap_cache(Some(map));

    //     Ok(None)
    // }
}

// impl APIRegister for CommandAPI {
//     fn register_methods(api: &mut super::API) {
//         let mut methods: HashMap<&str, APIMethod> = HashMap::new();
//         // methods.insert("command.run", Self::run_command);
//         methods.insert("command.register", Self::register_command);
//         methods.insert("command.test", Self::test);
//         methods.insert("keybind.register", Self::register_keybind);
//         api.register_api(methods);
//     }
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct RunCommandParams {
//     command: CommandRequest,
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// struct RegisterKeybind {
//     keys: Vec<String>,
//     command_id: Option<String>,
//     params: Option<Vec<Value>>,
// }

// struct RegisterCommandParams {
//     id: String,
//     doc_type: Option<DocType>,
//     function: CommandFunction,
// }
// fn parse_register_params(
//     input: &Option<ExternalCommandInput>,
// ) -> Result<RegisterCommandParams, String> {
//     match input {
//         Some(input) => match input {
//             ExternalCommandInput::Python(obj) => Python::attach(|py| {
//                 let bound_obj = obj.bind(py);

//                 let res = if bound_obj.is_callable() {
//                     bound_obj.call0().map_err(|e| e.to_string())?
//                 } else {
//                     bound_obj.clone()
//                 };

//                 // Extract fields directly from Python object
//                 let id: String = res
//                     .get_item("id")
//                     .map_err(|e| e.to_string())?
//                     .extract()
//                     .map_err(|e: PyErr| e.to_string())?;

//                 let doc_type_str: Option<String> = match res.get_item("doc_type") {
//                     Ok(item) => item.extract().ok(),
//                     Err(_) => None,
//                 };
//                 let doc_type: Option<DocType> = doc_type_str
//                     .map(|s| serde_json::from_value(serde_json::Value::String(s)))
//                     .transpose()
//                     .map_err(|e| format!("Invalid doctype: {}", e))?;

//                 let function = res
//                     .get_item("function")
//                     .map_err(|e| e.to_string())?
//                     .into_pyobject(py);
//                 Ok(RegisterCommandParams {
//                     id,
//                     doc_type,
//                     function: CommandFunction::Python(function.unwrap().unbind()),
//                 })
//             }),
//             ExternalCommandInput::JSON(value) => {
//                 Err("JSON input not supported for CommandParams with function".to_string())
//             }
//         },
//         None => Err("missing input parameters".to_string()),
//     }
// }
