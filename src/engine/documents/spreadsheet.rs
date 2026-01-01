use std::collections::{HashMap, HashSet};

pub struct SpreadSheet {
    pub cells: HashMap<usize, HashMap<usize, Cell>>,
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
