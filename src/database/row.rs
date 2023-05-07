use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Row {
    pub columns: HashMap<String, Value>,
}

impl Row {
    pub fn new(cols: HashMap<String, Value>) -> Row {
        Row { columns: cols }
    }
}
