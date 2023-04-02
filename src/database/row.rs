use serde_json::Value;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] 
pub struct Row {
  pub columns: HashMap<String, Value>
}

impl Row {
  pub fn new(cols: HashMap<String, Value>) -> Row {
    Row {columns: cols}
  }
}