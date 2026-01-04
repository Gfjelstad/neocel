use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn try_parse<T>(input: Option<Value>) -> Result<T, String>
where
    T: DeserializeOwned,
{
    if (input.is_none()) {
        return Err("missing input parameters".to_string());
    }
    serde_json::from_value::<T>(input.unwrap())
        .map_err(|e| format!("Failed to parse input params: {}", e))
}
