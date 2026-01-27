use std::{collections::HashMap, path::PathBuf, str::FromStr, sync::{Arc, Mutex}};

use pyo3::{PyErr, exceptions::PyRuntimeError, pyclass, pymethods};
use serde::Deserialize;
use serde_json::{json, to_value};

use crate::{api::PyValue, config::Theme, engine::{
        Engine, document::{Document, DocumentData}, documents::{DocumentDataProvider, text::TextDocumentData}
    }};


#[pyclass(unsendable)]
#[derive(Clone)]
pub struct ConfigAPI {
    engine: Arc<Mutex<Engine>>,
}

impl ConfigAPI {
    pub fn new(
        engine: Arc<Mutex<Engine>>,
    ) -> Self {
        Self {
            engine,
        }
    }
}

// pub struct CommandAPI {}

#[pymethods]
impl ConfigAPI {
    pub fn set_theme(&self, theme: HashMap<String,String>) -> Result<(), PyErr> {
        let new_theme = Theme::try_from( to_value(theme).map_err(|e| PyRuntimeError::new_err(e.to_string()))?).map_err(|e| PyRuntimeError::new_err(e))?;
        self.engine.lock().unwrap().config.theme = new_theme;
        
        Ok(())
    }
}

