use std::{path::PathBuf, str::FromStr};

use serde::Deserialize;
use serde_json::json;

use crate::{
    api::{APIMethodParams, APIMethodResult, APIRegister, utils},
    engine::{
        document::{Document, DocumentData},
        documents::{DocumentDataProvider, text::TextDocumentData},
    },
};

pub struct ConfigAPI {}

impl ConfigAPI {
    pub fn open_file(state: &mut APIMethodParams) -> APIMethodResult {
        let theme = utils::try_parse::<ThemeParams>(&state.params)?;
        Ok(None)
    }
}

impl APIRegister for ConfigAPI {
    fn register_methods(api: &mut super::API) {}
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
struct ThemeParams {
    background: String,
}
