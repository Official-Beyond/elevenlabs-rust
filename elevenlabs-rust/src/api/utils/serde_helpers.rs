use serde::{Serialize, Deserialize};
use serde_json::Result;

pub fn serialize<T: Serialize>(item: &T) -> Result<String> {
    serde_json::to_string(item)
}

pub fn deserialize<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T> {
    serde_json::from_str(s)
}
