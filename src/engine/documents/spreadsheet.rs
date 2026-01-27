use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

use csv::ReaderBuilder;
use pyo3::pyclass;
use serde::Serialize;

use crate::{engine::{
    WindowState,
    documents::{DocumentDataProvider, InsertModeProvider},
}, input::keymaps::Key};

#[derive(Debug, Serialize)]
pub struct SpreadSheetDocumentData {
    pub cells: HashMap<usize, HashMap<usize, Cell>>,
    // pub selected_cell: (usize, usize),
}
impl DocumentDataProvider for SpreadSheetDocumentData {
    fn new() -> Self {
        Self {
            cells: HashMap::new(),
            // selected_cell: (1, 1),
        }
    }

    fn from_file(path: &str) -> Result<Self, String> {
        let file = File::open(path.clone()).map_err(|e| e.to_string())?;
        let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

        let mut outer_map: HashMap<usize, HashMap<usize, Cell>> = HashMap::new();

        for (row_idx, result) in reader.records().enumerate() {
            let record = result.map_err(|e| e.to_string())?;
            let mut inner_map: HashMap<usize, Cell> = HashMap::new();

            for (col_idx, field) in record.iter().enumerate() {
                inner_map.insert(
                    col_idx,
                    Cell {
                        raw: field.to_string(),
                        value: CellValue::parse_from_str(field),
                        ast: None,
                        dependencies: HashSet::new(),
                        used_by: HashSet::new(),
                    },
                );
            }

            outer_map.insert(row_idx, inner_map);
        }
        Ok(Self {
            cells: outer_map,
            // selected_cell: (0, 0),
        })
    }

    fn from_raw(content: &str) -> Result<Self, String> {
        let mut reader = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(content.as_bytes());

        let mut outer_map: HashMap<usize, HashMap<usize, Cell>> = HashMap::new();

        for (row_idx, result) in reader.records().enumerate() {
            let record = result.map_err(|e| e.to_string())?;
            let mut inner_map: HashMap<usize, Cell> = HashMap::new();

            for (col_idx, field) in record.iter().enumerate() {
                inner_map.insert(
                    col_idx,
                    Cell {
                        raw: field.to_string(),
                        value: CellValue::parse_from_str(field),
                        ast: None,
                        dependencies: HashSet::new(),
                        used_by: HashSet::new(),
                    },
                );
            }

            outer_map.insert(row_idx, inner_map);
        }
        Ok(Self {
            cells: outer_map,
            // selected_cell: (0, 0),
        })
    }
}
impl InsertModeProvider for SpreadSheetDocumentData {
    fn handle_key(
        &mut self,
        window: &mut WindowState,
        key: Key,
    ) -> Result<(), String> {
        Ok(())
    }
}
pub type CellId = String;

#[derive(Debug, Serialize, Clone)]
#[pyclass]
pub struct Cell {
    pub raw: String,
    pub value: CellValue,
    pub ast: Option<Expr>,
    pub dependencies: HashSet<CellId>,
    pub used_by: HashSet<CellId>,
}

impl Default for Cell {
    fn default() -> Self {
        Cell {
            raw: String::new(),
            value: CellValue::Empty,
            ast: None,
            dependencies: HashSet::new(),
            used_by: HashSet::new(),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
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
#[derive(Debug, Serialize, Clone)]
pub struct Expr {}
