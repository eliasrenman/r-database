use super::row::Row;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Index {
    desc_sort: bool,
    key: String,
    unique: bool,
    index_rows: HashMap<String, Vec<u64>>,
}

impl Index {
    pub fn new(key: String, desc_sort: Option<bool>, unique: Option<bool>) -> Index {
        Index {
            desc_sort: desc_sort.unwrap_or(false),
            key,
            unique: unique.unwrap_or(false),
            index_rows: hashmap!(),
        }
    }

    pub fn insert_row(&mut self, pk_name: String, row: Row) -> Result<(), &str> {
        let row_value = row.get(&self.key);

        // Check if the index row already exists
        let index_row = if row_value.is_some() {
            let str = serde_json::to_value(row_value.unwrap());

            self.index_rows.get(&str.unwrap().to_string())
        } else {
            Option::None
        };

        let key = serde_json::to_value(row_value.unwrap())
            .unwrap()
            .to_string();

        let pk_value = row.get(&pk_name).unwrap().as_u64().unwrap();

        if index_row.is_none() {
            // Insert the new row into the index
            self.index_rows.insert(key, vec![pk_value]);
            return Result::Ok(());
        }

        if self.unique {
            return Result::Err("Index is unique!");
        }

        let mut vector = index_row.unwrap().clone();

        vector.push(pk_value);

        vector.sort();

        if self.desc_sort {
            vector.reverse();
        }

        self.index_rows.insert(key, vector);

        return Result::Ok(());
    }

    pub fn get_pks_by_value(&self, value: Value) -> Option<&Vec<u64>> {
        let key = serde_json::to_value(value).unwrap().to_string();

        self.index_rows.get(&key)
    }
}

#[derive(Serialize, Deserialize)]
pub struct IndexRow {
    pk: u64,
    value: String,
}
