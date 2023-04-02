
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] 
pub struct Index {
  pk: u64,
  key: String,
  asd_sort: bool,
  unique: bool,

}