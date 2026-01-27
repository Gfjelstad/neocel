use crate::{
    commands::{command_dispatcher::CommandRequest},
    engine::{
        Edit,
        documents::{
            InsertModeProvider, spreadsheet::SpreadSheetDocumentData, text::TextDocumentData,
        },
    },
    input::keymaps::{ActionNode},
};
use pyo3::pyclass;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use uuid::Uuid;

pub type DocId = String;

#[derive(Debug, Eq, Clone, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[pyclass]
pub enum DocType {
    SpreadSheet,
    Info,
    Text,
}
#[derive(Serialize)]
pub struct Document {
    pub id: DocId,
    pub doc_type: DocType,
    pub path: Option<PathBuf>,
    pub data: DocumentData,
    #[serde(skip)]
    pub undo_stack: Vec<Edit>,
}
impl Document {
    pub fn new(data: DocumentData, path: Option<PathBuf>) -> (DocId, Self) {
        let doc_type = match data {
            DocumentData::Text(_) => DocType::Text,
            DocumentData::SpreadSheet(_) => DocType::SpreadSheet,
            DocumentData::Help(_) => DocType::Info,
            _ => DocType::Info,
        };
        let id = Uuid::new_v4().to_string();
        (
            id.clone(),
            Self {
                id,
                doc_type: doc_type.clone(),
                path,
                data,
                undo_stack: vec![],
            },
        )
    }
}

#[derive(Serialize)]
pub enum DocumentData {
    SpreadSheet(SpreadSheetDocumentData),
    Text(TextDocumentData),
    Help(String),
    Config(String),
}
impl DocumentData {
    pub fn as_insertable(&mut self) -> Option<&mut dyn InsertModeProvider> {
        match self {
            Self::SpreadSheet(t) => Some(t),
            Self::Text(t) => Some(t),
            _ => None,
        }
    }
}
