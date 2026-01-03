use std::collections::{HashMap, HashSet};

use serde::Serialize;

use crate::engine::{
    WindowState,
    documents::{DocumentDataProvider, InsertModeProvider},
};

#[derive(Debug, Serialize)]
pub struct SpreadSheetDocumentData {
    pub cells: HashMap<usize, HashMap<usize, Cell>>,
}
impl DocumentDataProvider for SpreadSheetDocumentData {
    fn new(data: &str) -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}
impl InsertModeProvider for SpreadSheetDocumentData {
    fn handle_key(
        &mut self,
        window: &mut WindowState,
        key: crate::commands::Key,
    ) -> Result<(), String> {
        Ok(())
    }
}
pub type CellId = String;

#[derive(Debug, Serialize)]
pub struct Cell {
    pub raw: String,
    pub value: CellValue,
    pub ast: Option<Expr>,
    pub dependencies: HashSet<CellId>,
    pub used_by: HashSet<CellId>,
}

#[derive(Debug, Serialize)]
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
#[derive(Debug, Serialize)]
pub struct Expr {}
