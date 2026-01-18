use pyo3::{Py, PyAny, Python, types::PyAnyMethods};
use pythonize::depythonize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::api::ExternalCommandInput;

pub fn try_parse<T>(input: &Option<ExternalCommandInput>) -> Result<T, String>
where
    T: DeserializeOwned,
{
    match input {
        Some(input) => {
            match input {
                ExternalCommandInput::Python(obj) => Python::attach(|py| {
                    let bound_obj = obj.bind(py);
                    let res = if bound_obj.is_callable() {
                        let result = bound_obj.call0().map_err(|e| e.to_string())?;
                        result
                    } else {
                        bound_obj.clone()
                    };
                    // 2. Try to deserialize into Command
                    let params: T = depythonize(&res).map_err(|e| e.to_string())?;
                    Ok(params)
                }),
                ExternalCommandInput::JSON(value) => serde_json::from_value::<T>(value.clone())
                    .map_err(|e| format!("Failed to parse input params: {}", e)),
            }
        }
        None => Err("missing input parameters".to_string()),
    }
}
