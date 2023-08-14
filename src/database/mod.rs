use std::{borrow::BorrowMut, fs, ops::Deref};

use serde::{Deserialize, Serialize};

use self::{relation::Relation, table::Table};

pub mod index;
pub mod relation;
pub mod row;
pub mod table;

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new(tables: Vec<Table>) -> Database {
        Database { tables: tables }
    }

    pub fn from_file(file_path: String) -> Result<Database, &'static str> {
        let serialized = fs::read_to_string(file_path);
        if serialized.is_err() {
            return Err("Failed loading from file");
        }
        let database: Database = serde_json::from_str(&serialized.unwrap()).unwrap();
        return Ok(database);
    }

    pub fn to_file(&self, file_path: &'static str) {
        let serialized = serde_json::to_string(self).unwrap();
        println!("serialized = {}", serialized);
        let result = fs::write(file_path, serialized);
        if result.is_err() {
            panic!("Failed writing table to file")
        }
    }

    pub fn get_table(&mut self, name: String) -> Result<&mut Table, String> {
        for table in self.tables.iter_mut() {
            if table.name == name {
                return Ok(table);
            }
        }
        Err("Failed to find table".to_string())
    }

    pub fn get_table_relation(
        &self,
        table_name: String,
        relation_name: &String,
    ) -> Result<&Relation, String> {
        for table in &self.tables {
            if table.name == table_name {
                let relation = table.get_relation(relation_name).unwrap();
                return Ok(relation);
            }
        }
        Err("Failed to find table".to_string())
    }
}
