pub mod text_document_api;
pub mod sheet_document_api;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use pyo3::{PyErr, pyclass, pymethods};
use serde::Deserialize;

use crate::{
    api::PyValue, engine::{Engine, documents::spreadsheet::Cell}, input::input_engine::{InputEngine, Mode}
};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct DocumentAPI {
    input_engine: Arc<Mutex<InputEngine>>,
    engine: Arc<Mutex<Engine>>
}

impl DocumentAPI {
    pub fn new(input_engine: Arc<Mutex<InputEngine>>,engine: Arc<Mutex<Engine>>) -> Self {
        Self { input_engine, engine }
    }
}

#[pymethods]
impl DocumentAPI {
    pub fn change_mode(&self, mode: PyValue) -> Result<(), PyErr> {
        let mode = mode.parse_to::<Mode>()?;
        self.input_engine.lock().unwrap().change_mode(mode);
        Ok(())
    }

    pub fn get_cursor(&self)  -> Result<(usize, usize), PyErr> {
        let mut engine_guard = self.engine.lock().unwrap();
        let (win, _) = engine_guard.get_current_window();
        Ok((win.cursor_row, win.cursor_col))
    }

    pub fn set_cursor(&self, row: usize, col: usize) -> Result<(), PyErr> {
        let mut engine_guard = self.engine.lock().unwrap();
        let (win, doc) = engine_guard.get_current_window();

        let (row, col) = (row.max(0), col.max(0));

        // if (doc.doc_type != DocType::SpreadSheet) {};
        match &mut doc.data {
            crate::engine::document::DocumentData::SpreadSheet(spread_sheet_document_data) => {
                win.cursor_col = col;
                win.cursor_row = row;
                spread_sheet_document_data
                    .cells
                    .entry(row)
                    .or_insert_with(HashMap::new)
                    .entry(col)
                    .or_insert_with(Cell::default);
                Ok(())
            },
            crate::engine::document::DocumentData::Text(text_document_data) => {
                let row = row.min(text_document_data.data.len() - 1);
                let col = col.min(text_document_data.data[row].len() - 1);
                win.cursor_row = row;
                win.cursor_col = col;
                Ok(())
            },
            crate::engine::document::DocumentData::Help(_) => todo!(),
            crate::engine::document::DocumentData::Config(_) => todo!(),
        }
        // if let DocumentData::SpreadSheet(data) = &mut doc.data {
        //     // data.selected_cell = (row, col);
            

        // } else {
        //     return Err(PyRuntimeError::new_err(
        //         "active document must be SpreadSheet to navigate",
        //     ));
        // }
        
    }
}


#[pyclass]
struct ChangeModeParams {
    mode: Mode,
}
