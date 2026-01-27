use std::{
    collections::HashMap,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};

use pyo3::{
    PyErr,
    exceptions::{PyRuntimeError, PyValueError},
    pyclass, pymethods,
};
use serde::Deserialize;
use serde_json::json;

use crate::engine::{
    Engine,
    document::{DocType, Document, DocumentData},
    documents::{
        DocumentDataProvider,
        spreadsheet::{Cell, SpreadSheetDocumentData},
        text::TextDocumentData,
    },
};

#[pyclass(unsendable)]
#[derive(Clone)]
pub struct SheetDocumentAPI {
    engine: Arc<Mutex<Engine>>,
}

impl SheetDocumentAPI {
    pub fn new(engine: Arc<Mutex<Engine>>) -> Self {
        Self { engine }
    }
}

// pub struct CommandAPI {}

#[pymethods]
impl SheetDocumentAPI {
    #[pyo3(signature = (content=None, path=None))]
    pub fn open_file(
        &self,
        content: Option<String>,
        path: Option<String>,
    ) -> Result<String, PyErr> {
        if (content.is_none() && path.is_none()) || (content.is_some() && path.is_some()) {
            return Err(PyValueError::new_err("only one of foo or bar is required"));
        }

        let doc = if let Some(content) = content {
            Some(Document::new(
                DocumentData::SpreadSheet(
                    SpreadSheetDocumentData::from_raw(content.as_str())
                        .map_err(|e| PyRuntimeError::new_err(e))?,
                ),
                None,
            ))
        } else if let Some(path) = path {
            Some(Document::new(
                DocumentData::Text(
                    TextDocumentData::from_file(path.as_str())
                        .map_err(|e| PyRuntimeError::new_err(e))?,
                ),
                Some(
                    PathBuf::from_str(path.as_str())
                        .map_err(|e| e.to_string())
                        .map_err(|e| PyRuntimeError::new_err(e))?,
                ),
            ))
        } else {
            None
        };
        if let Some((doc_id, doc)) = doc {
            self.engine.lock().unwrap().docs.insert(doc_id.clone(), doc);

            return Ok(doc_id);
        } else {
            return Err(PyRuntimeError::new_err("could not open file"));
        }
    }
    pub fn location(&self) -> Result<(usize, usize), PyErr> {
        let mut engine_guard = self.engine.lock().unwrap();
        let (win, doc) = engine_guard.get_current_window();

        Ok((win.cursor_row, win.cursor_col))
    }

    
}

// impl APIRegister for TextDocumentAPI {
//     fn register_methods(api: &mut super::API) {}
// }

// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// enum OpenFileParams {
//     Path { path: String },
//     Raw { content: String },
// }
