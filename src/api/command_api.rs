use std::collections::HashMap;

use pyo3::{IntoPyObject, PyErr, Python, types::PyAnyMethods};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    api::{
        APIMethod, APIMethodParams, APIMethodResult, APIRegister, ExternalCommandInput,
        utils::try_parse,
    },
    commands::{
        command_dispatcher::{CommandFunction, CommandRequest},
        insert_into_tree,
    },
    engine::document::DocType,
    input::{Token, keymaps::KeymapProvider},
};

pub struct CommandAPI {}

impl CommandAPI {
    // pub fn run_command(state: &mut APIMethodParams) -> APIMethodResult {
    //     let command = try_parse::<RunCommandParams>(&state.params)?.command;

    //     state
    //         .command_dispatch
    //         .dispatch(&command, state.engine, state.input_engine, state.ui)
    // }

    pub fn test(state: &mut APIMethodParams) -> APIMethodResult {
        println!("HELLO FROM THE API");
        Ok(None)
    }

    pub fn register_command(state: &mut APIMethodParams) -> APIMethodResult {
        let command = parse_register_params(&state.params)?;

        match command.doc_type {
            Some(doc_type) => {
                state.command_dispatch.register_for_doc(
                    doc_type,
                    command.id.as_str(),
                    command.function,
                );
            }
            None => {
                state
                    .command_dispatch
                    .register_global(command.id.as_str(), command.function);
            }
        }

        Ok(None)
    }

    pub fn register_keybind(state: &mut APIMethodParams) -> APIMethodResult {
        let command = try_parse::<RegisterKeybind>(&state.params)?;

        // let tree = build_keymap_tree(&command.keys, Token::Command(CommandRequest { id: command.command_id.unwrap().clone(), args: command.params }))?;

        let mut map = state.engine.keymap().clone();

        if command.command_id.is_some() {
            insert_into_tree(
                &mut map,
                &command.keys,
                Token::Command({
                    CommandRequest {
                        id: command.command_id.unwrap().clone(),
                        args: command.params.unwrap_or_default(),
                    }
                }),
            )?;
        }

        state.engine.set_keymap_cache(Some(map));

        Ok(None)
    }
}

impl APIRegister for CommandAPI {
    fn register_methods(api: &mut super::API) {
        let mut methods: HashMap<&str, APIMethod> = HashMap::new();
        // methods.insert("command.run", Self::run_command);
        methods.insert("command.register", Self::register_command);
        methods.insert("command.test", Self::test);
        methods.insert("keybind.register", Self::register_keybind);
        api.register_api(methods);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct RunCommandParams {
    command: CommandRequest,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct RegisterKeybind {
    keys: Vec<String>,
    command_id: Option<String>,
    params: Option<Vec<Value>>,
}

struct RegisterCommandParams {
    id: String,
    doc_type: Option<DocType>,
    function: CommandFunction,
}
fn parse_register_params(
    input: &Option<ExternalCommandInput>,
) -> Result<RegisterCommandParams, String> {
    match input {
        Some(input) => match input {
            ExternalCommandInput::Python(obj) => Python::attach(|py| {
                let bound_obj = obj.bind(py);

                let res = if bound_obj.is_callable() {
                    bound_obj.call0().map_err(|e| e.to_string())?
                } else {
                    bound_obj.clone()
                };

                // Extract fields directly from Python object
                let id: String = res
                    .get_item("id")
                    .map_err(|e| e.to_string())?
                    .extract()
                    .map_err(|e: PyErr| e.to_string())?;

                let doc_type_str: Option<String> = match res.get_item("doc_type") {
                    Ok(item) => item.extract().ok(),
                    Err(_) => None,
                };
                let doc_type: Option<DocType> = doc_type_str
                    .map(|s| serde_json::from_value(serde_json::Value::String(s)))
                    .transpose()
                    .map_err(|e| format!("Invalid doctype: {}", e))?;

                let function = res
                    .get_item("function")
                    .map_err(|e| e.to_string())?
                    .into_pyobject(py);
                Ok(RegisterCommandParams {
                    id,
                    doc_type,
                    function: CommandFunction::Python(function.unwrap().unbind()),
                })
            }),
            ExternalCommandInput::JSON(value) => {
                Err("JSON input not supported for CommandParams with function".to_string())
            }
        },
        None => Err("missing input parameters".to_string()),
    }
}
