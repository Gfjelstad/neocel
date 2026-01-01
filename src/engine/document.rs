use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use uuid::Uuid;

use crate::engine::{Edit, documents::spreadsheet::SpreadSheet};

pub type DocId = String;

#[derive(Eq, Hash, PartialEq)]
pub enum DocType {
    SpreadSheet,
    Info,
    Text,
}
pub struct Document {
    pub doc_type: DocType,
    pub path: Option<PathBuf>,
    pub data: DocumentData,
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
        (
            Uuid::new_v4().to_string(),
            Self {
                doc_type,
                path,
                data,
                undo_stack: vec![],
            },
        )
    }
}
pub enum DocumentData {
    SpreadSheet(SpreadSheet),
    Text(Vec<String>),
    Help(String),
    Config(String),
}
