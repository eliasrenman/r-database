use super::{index::Index, relation::Relation, row::Row};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    borrow::Borrow,
    collections::{hash_map::Entry, HashMap},
};

#[derive(Serialize, Deserialize)]
pub struct Table {
    indexes: HashMap<String, Index>,
    rows: HashMap<u64, Row>,
    pub name: String,
    pk: String,
    relations: HashMap<String, Relation>,
}

impl Table {
    pub fn new(
        name: &str,
        pk: &str,
        indexes: Option<HashMap<String, Index>>,
        relations: Option<HashMap<String, Relation>>,
    ) -> Table {
        Table {
            indexes: indexes.unwrap_or(HashMap::new()),
            rows: HashMap::new(),
            name: name.to_string(),
            pk: pk.to_string(),
            relations: relations.unwrap_or(HashMap::new()),
        }
    }

    pub fn create_index(&mut self, key: String, mut index: Index) {
        for ele in self.rows.clone() {
            let _ = index.insert_row(self.pk.clone(), ele.1.clone());
        }
        self.indexes.insert(key, index);
    }

    pub fn insert_row(&mut self, row: Row) -> Result<(), String> {
        let key_option = row.get(&self.pk);

        if key_option.is_none() {
            return Err("Primary key not found on row to insert!".to_string());
        }

        let row_primary_key = key_option.unwrap().as_u64();

        if row_primary_key.is_none() {
            return Err("Primary key is not of type u64".to_string());
        }

        if self.rows.contains_key(row_primary_key.unwrap().borrow()) {
            return Err("Primary key already exists".to_string());
        }

        // Insert row into database
        self.rows.insert(row_primary_key.unwrap(), row.clone());

        // Insert row into eventual indexes
        for (key, _) in row.clone() {
            let index_key = key.as_str().to_string();

            match self.indexes.entry(index_key) {
                Entry::Occupied(mut e) => {
                    let result = e.get_mut().insert_row(self.pk.clone(), row.clone());
                    if result.is_err() {
                        return Err(format!(
                            "Failed writing to index with erro: {}",
                            result.err().unwrap()
                        ));
                    }
                }
                Entry::Vacant(_e) => {}
            }
        }

        Result::Ok(())
    }

    pub fn find_by_pk(&self, value: u64) -> Result<&Row, String> {
        let row = self.rows.get(&value);
        if row.is_none() {
            return Err(format!("Row with pk: {} not found!", value));
        }
        Ok(row.unwrap())
    }

    pub fn find_by_pks(&self, value: Vec<u64>) -> Vec<Result<&Row, String>> {
        let mut vector = vec![];
        for ele in value {
            vector.push(self.find_by_pk(ele));
        }
        return vector;
    }

    pub fn get_pks_by_index(&self, index_key: String, value: Value) -> Result<Vec<u64>, String> {
        // Attempt to get the primary keys for the index.

        if !self.indexes.contains_key(&index_key) {
            // If it fails then return a error

            let formatted_error =
                format!("Nothing found index not found: {}", index_key).to_string();
            return Err(formatted_error);
        }

        return Ok(self
            .indexes
            .get(&index_key)
            .unwrap()
            .get_pks_by_value(value.clone())
            .unwrap()
            .clone());
    }

    pub fn get_relation(&self, column_name: &String) -> Result<&Relation, String> {
        let relation = self.relations.get(column_name);

        if !relation.is_none() {
            return Ok(relation.unwrap());
        }
      Err(format!("Failed to find relation, '{column_name}'").to_string())
    }
}
