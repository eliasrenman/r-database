use super::{index::Index, row::Row};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Serialize, Deserialize)]
pub struct Table {
    indexes: Vec<Index>,
    rows: HashMap<u64, Row>,
    name: String,
    pk_key: String,
}

impl Table {
    pub fn new(name: &str, pk_key: &str) -> Table {
        Table {
            indexes: vec![],
            rows: HashMap::new(),
            name: name.to_string(),
            pk_key: pk_key.to_string(),
        }
    }

    pub fn create_index(&mut self, index: Index) {
        // TODO: Add row indexes to the index itself before pushing to the table
        self.indexes.push(index)
    }

    pub fn find_by_pk(&self, value: &u64) -> Result<&Row, &'static str> {
        let row = self.rows.get(value);
        if row.is_none() {
            return Err("Row not found!");
        }
        Ok(row.unwrap().clone())
    }

    pub fn insert_row(&mut self, row: Row) {
        let key_option = row.columns.get(&self.pk_key);

        if key_option.is_none() {
            panic!("Primary key not found on row to insert!");
        }

        let key = key_option.unwrap().as_u64();

        if key.is_none() {
            panic!("Primary key is not of type u64");
        }

        self.rows.insert(key.unwrap(), row);
    }
}
