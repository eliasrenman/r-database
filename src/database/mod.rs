use std::{collections::HashMap, fs};

use serde::{Deserialize, Serialize};

use self::table::Table;

pub mod index;
pub mod row;
pub mod table;

#[derive(Serialize, Deserialize)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(tables: HashMap<String, Table>) -> Database {
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
}

#[cfg(test)]
mod tests {
    use crate::database::{row::Row, table::Table, Database};
    use std::{fs, path::Path};

    #[test]
    fn should_write_and_read_to_file() {
        let mut table: Table = Table::new("Cats", "id");
        table.insert_row(Row::new(hashmapJson!["id" => 1, "name" => "Ozzy"]));
        table.insert_row(Row::new(hashmapJson!["id" => 2, "name" => "Simon"]));
        let database = Database::new(hashmap!["Cats" => table]);

        database.to_file("./db.json");

        let exists = Path::try_exists(Path::new("./db.json"));
        assert_eq!(exists.unwrap(), true);

        let table = Database::from_file("./db.json".to_owned());
        assert_eq!(table.is_ok(), true);

        let db = table.unwrap();
        let table = db.tables.get("Cats");
        if table.is_none() {
            panic!("Unable to find table");
        }

        let row = table.unwrap().find_by_pk(&1u64);
        assert_eq!(row.is_ok(), true);

        // Cleanup file
        let _result = fs::remove_file("./db.json");
    }
}
