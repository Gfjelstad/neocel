use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn try_parse<T>(input: Value) -> Result<T, String>
where
    T: DeserializeOwned,
{
    serde_json::from_value::<T>(input).map_err(|e| format!("Failed to parse input params: {}", e))
}
