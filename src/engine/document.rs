use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use uuid::Uuid;

use crate::engine::Edit;

pub type DocId = String;
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
    SpreadSheet(HashMap<usize, HashMap<usize, Cell>>),
    Text(Vec<String>),

    Help(String),
    Config(String),
}
pub type CellId = String;
pub struct Cell {
    pub raw: String,
    pub value: CellValue,
    pub ast: Option<Expr>,
    pub dependencies: HashSet<CellId>,
    pub used_by: HashSet<CellId>,
}
pub enum CellValue {
    Empty,
    Number(f64),
    Text(String),
    Error(String),
}

impl CellValue {
    pub fn parse_from_str(s: &str) -> Self {
        let trimmed = s.trim();

        // Check if empty
        if trimmed.is_empty() {
            return CellValue::Empty;
        }

        // Try to parse as number
        match trimmed.parse::<f64>() {
            Ok(num) => CellValue::Number(num),
            Err(_) => CellValue::Text(trimmed.to_string()),
        }
    }
}

pub struct Expr {}
