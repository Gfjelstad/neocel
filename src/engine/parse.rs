use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::path::PathBuf;

use crate::engine::document::{DocId, Document};
use crate::engine::documents::DocumentDataProvider;
use crate::engine::documents::spreadsheet::{Cell, CellValue, SpreadSheetDocumentData};
pub fn parse_csv_to_doc(path: PathBuf) -> Result<(DocId, Document), Box<dyn std::error::Error>> {
    let file = File::open(path.clone())?;
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);

    let mut outer_map: HashMap<usize, HashMap<usize, Cell>> = HashMap::new();

    for (row_idx, result) in reader.records().enumerate() {
        let record = result?;
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

    Ok(Document::new(
        crate::engine::DocumentData::SpreadSheet(SpreadSheetDocumentData {
            cells: outer_map
        }),
        Some(path),
    ))
}
