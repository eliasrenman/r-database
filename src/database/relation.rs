use super::{row::Row, table::Table, Database};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Relation {
    pub table_name: String,
    pub variation: String,
}
impl Relation {
    pub fn new(table_name: String, variation: String) -> Relation {
        Relation {
            table_name,
            variation,
        }
    }

    pub fn get_foreign_row(&self, database: &Database, foreign_id: u64) -> Result<Row, String> {
        // Find foreign table
        let table = database.get_table(self.table_name.clone()).unwrap();
        // Get row from foreign table
        let row = table.find_by_pk(foreign_id).unwrap().clone();

        Ok(row)
    }
}

#[derive(Serialize, Deserialize)]
pub struct OneToOne {
    foreign_id: u64,
    relation_name: String,
}

impl OneToOne {
    pub fn new(foreign_id: u64, relation_name: String) -> OneToOne {
        OneToOne {
            foreign_id,
            relation_name,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.foreign_id
    }

    pub fn get(&self, table_name: String, database: &Database) -> Result<Row, String> {
        let relation_result: Result<&Relation, String> = {
            // Get table
            let table: &Table = match database.get_table(table_name) {
                Ok(relation) => relation,
                Err(error) => return Err(error),
            };

            // Fetch the Relation
            table.get_relation(&self.relation_name)
        };

        let relation: Relation = match relation_result {
            Ok(relation) => relation.clone(),
            Err(error) => return Err(error),
        };

        // Fetch row from foreign table
        relation.get_foreign_row(database, self.foreign_id)
    }

    pub fn from_value(value: Value) -> OneToOne {
        serde_json::from_value(value).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub struct OneToMany {
    foreign_ids: Vec<u64>,
    relation_name: String,
}
impl OneToMany {
    pub fn new(foreign_ids: Vec<u64>, relation_name: String) -> OneToMany {
        OneToMany {
            foreign_ids,
            relation_name
        }
    }

    pub fn get_ids(&self) -> Vec<u64> {
        self.foreign_ids.clone()
    }

    pub fn get(&self, table_name: String, database: &Database) -> Result<Vec<Row>, String> {
        let relation_result: Result<&Relation, String> = {
            let table: &Table = match database.get_table(table_name) {
                Ok(relation) => relation,
                Err(error) => return Err(error),
            };

            // Fetch the Relation
            table.get_relation(&self.relation_name)
        };

        let relation: Relation = match relation_result {
            Ok(relation) => relation.clone(),
            Err(error) => return Err(error),
        };

        let rows = self.foreign_ids.iter().map(|id| {
            relation.get_foreign_row(database, *id).unwrap()
        });
        Ok(rows.collect())
    }

    pub fn from_value(value: Value) -> OneToMany {
      serde_json::from_value(value).unwrap()
    }
}
