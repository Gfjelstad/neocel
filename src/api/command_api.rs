use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    api::{APIMethod, APIMethodParams, APIMethodResult, APIRegister, utils::try_parse},
    commands::command_dispatcher::Command,
};

pub struct CommandAPI {}

impl CommandAPI {
    pub fn run_command(state: &mut APIMethodParams) -> APIMethodResult {
        let command = try_parse::<RunCommandParams>(state.params.clone())?.command;

        state
            .command_dispatch
            .dispatch(&command, state.engine, state.input_engine, state.ui)
    }

    pub fn register_command(state: &mut APIMethodParams) -> APIMethodResult {
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
    command: Command,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct RegisterCommandParams {
    id: String,
    command: Command,
}

