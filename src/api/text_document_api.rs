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

pub struct TextDocumentAPI {}

impl TextDocumentAPI {
    pub fn open_file(state: &mut APIMethodParams) -> APIMethodResult {
        let dir = utils::try_parse::<OpenFileParams>(&state.params)?;
        let res = match dir {
            OpenFileParams::Path { path } => Document::new(
                DocumentData::Text(TextDocumentData::from_file(path.as_str())?),
                Some(PathBuf::from_str(path.as_str()).map_err(|e| e.to_string())?),
            ),
            OpenFileParams::Raw { content } => Document::new(
                DocumentData::Text(TextDocumentData::from_raw(content.as_str())?),
                None,
            ),
        };
        state.engine.docs.insert(res.0.clone(), res.1);

        Ok(Some(json!({"documentId":res.0.clone()})))
    }
}

impl APIRegister for TextDocumentAPI {
    fn register_methods(api: &mut super::API) {}
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum OpenFileParams {
    Path { path: String },
    Raw { content: String },
}

