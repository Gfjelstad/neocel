pub mod command_api;
pub mod engine_api;
pub mod documents;
pub mod config_api;
pub mod keybind_api;
use std::
    sync::{Arc, Mutex}
;

use pyo3::{
    Bound, BoundObject, FromPyObject, IntoPyObject, Py, PyAny, PyClass, PyClassInitializer, PyErr, PyErrArguments, PyResult, Python, exceptions::{PyRuntimeError, PyTypeError}, pyclass, types::{PyModule, PyModuleMethods}
};
use pythonize::depythonize;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    api::{command_api::CommandAPI, config_api::ConfigAPI, documents::{DocumentAPI, text_document_api::TextDocumentAPI}, engine_api::EngineAPI2, keybind_api::KeybindApi},
    commands::command_dispatcher::CommandDispatcher,
    engine::Engine,
    input::input_engine::InputEngine,
};


pub struct API2 {
    pub engine_api: Arc<EngineAPI2>,
    pub command_api: Arc<CommandAPI>,
    pub config_api: Arc<ConfigAPI>,
    pub text_document_api: Arc<TextDocumentAPI>,
    pub document_api: Arc<DocumentAPI>,
    pub keybind_api: Arc<KeybindApi>

}

impl API2 {
    pub fn new(
        engine: Arc<Mutex<Engine>>,
        command_dispatch: Arc<Mutex<CommandDispatcher>>,
        input_engine: Arc<Mutex<InputEngine>>
    ) -> Self {
        Self {
            engine_api: Arc::new(EngineAPI2::new(engine.clone())),
            command_api: Arc::new(CommandAPI::new(command_dispatch.clone(), input_engine.clone(), engine.clone())),
            config_api: Arc::new(ConfigAPI::new(engine.clone())),
            text_document_api: Arc::new(TextDocumentAPI::new(engine.clone())),
            document_api: Arc::new(DocumentAPI::new(input_engine.clone(), engine.clone())),
            keybind_api: Arc::new(KeybindApi::new(engine.clone(), command_dispatch.clone(), input_engine.clone()))
        }
    }

 
    fn bind<T>( py: Python, module: &Bound<'_, PyModule>, name: &str, instance: &Arc<T>) -> PyResult<()>
    where
        T: PyClass + Clone + Into<PyClassInitializer<T>>,
    {
        module.add_class::<T>()?;
        let py_instance = Py::new(py, (**instance).clone())?;
        module.add(name, py_instance)?;
        Ok(())
    }

    pub fn to_module<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyModule>> {

        let module: Bound<'_, PyModule> = PyModule::new(py, "api")?;
   
        API2::bind(py, &module, "engine", &self.engine_api)?;
        API2::bind(py, &module, "commands", &self.command_api)?;
        API2::bind(py, &module, "config", &self.config_api)?;
        API2::bind(py, &module, "text", &self.text_document_api)?;
        API2::bind(py, &module, "document", &self.document_api)?;
        API2::bind(py, &module, "keybinds", &self.keybind_api)?;

        Ok(module)
    }
}



// #[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct PyValue(pub Value);

impl PyValue {
    pub fn parse_to<T: for<'de> Deserialize<'de>>(&self) -> Result<T, PyErr> {
        serde_json::from_value::<T>(self.0.clone())
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

}


impl<'a, 'py> FromPyObject<'a, 'py> for PyValue {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        depythonize(&*obj)
            .map(PyValue)
            .map_err(|e: pythonize::PythonizeError| PyTypeError::new_err(e.to_string()))
    }
}
impl<'py> IntoPyObject<'py> for PyValue {
    type Target = pyo3::PyAny;
    type Output = Bound<'py, pyo3::PyAny>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        pythonize::pythonize(py, &self).map_err(|e| PyTypeError::new_err(e.to_string()))
    }
}
// impl<'a, 'py> FromPyObject<'a, 'py> for PyValue {
//     type Error = PyErr;
//     fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
//         let params: Value = depythonize(&obj).map_err(|e| PyTypeError::new_err(e.to_string()))?;
//         Ok(PyValue(params))
//     }
// }