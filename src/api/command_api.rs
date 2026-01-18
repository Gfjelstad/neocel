use std::collections::HashMap;

use pyo3::{IntoPyObject, Py, PyAny, PyErr, Python, types::PyAnyMethods};
use serde::Deserialize;

use crate::{
    api::{
        APIMethod, APIMethodParams, APIMethodResult, APIRegister, ExternalCommandInput,
        utils::try_parse,
    },
    commands::command_dispatcher::{CommandFunction, CommandRequest},
    engine::document::DocType,
};

pub struct CommandAPI {}

impl CommandAPI {
    pub fn run_command(state: &mut APIMethodParams) -> APIMethodResult {
        let command = try_parse::<RunCommandParams>(&state.params)?.command;

        state
            .command_dispatch
            .dispatch(&command, state.engine, state.input_engine, state.ui)
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
}

impl APIRegister for CommandAPI {
    fn register_methods(api: &mut super::API) {
        let mut methods: HashMap<&str, APIMethod> = HashMap::new();
        methods.insert("doc.changeMode", Self::run_command);
        api.register_api(methods);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct RunCommandParams {
    command: CommandRequest,
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
                    .getattr("id")
                    .map_err(|e| e.to_string())?
                    .extract()
                    .map_err(|e: PyErr| e.to_string())?;

                let doc_type_str: Option<String> = res
                    .getattr("dottype")
                    .map_err(|e| e.to_string())?
                    .extract()
                    .map_err(|e: PyErr| e.to_string())?;

                let doc_type: Option<DocType> = doc_type_str
                    .map(|s| serde_json::from_value(serde_json::Value::String(s)))
                    .transpose()
                    .map_err(|e| format!("Invalid dottype: {}", e))?;

                let function = res
                    .getattr("function")
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
