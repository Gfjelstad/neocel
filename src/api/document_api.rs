use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    api::{APIMethod, APIMethodParams, APIMethodResult, APIRegister, utils::try_parse},
    input::input_engine::{Mode, ModeType},
};

pub struct DocumentAPI {}

impl DocumentAPI {
    pub fn change_mode(state: &mut APIMethodParams) -> APIMethodResult {
        let imode = try_parse::<ChangeModeParams>(state.params.clone())?;
        state.input_engine.mode = Mode::new(imode.mode);
        Ok(None)
    }
}

impl APIRegister for DocumentAPI {
    fn register_methods(api: &mut super::API) {
        let mut methods: HashMap<&str, APIMethod> = HashMap::new();
        methods.insert("doc.changeMode", Self::change_mode);
        api.register_api(methods);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct ChangeModeParams {
    mode: ModeType,
}
