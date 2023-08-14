use std::{borrow::BorrowMut, cell::RefCell, ops::DerefMut};

use super::{row::Row, table::Table, Database};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Serialize, Deserialize)]
pub struct Relation {
    pub table_name: String,
    variation: String,
}
impl Relation {
    pub fn new(table_name: String, variation: String) -> Relation {
        Relation {
            table_name,
            variation,
        }
    }

    pub fn get_foreign_row(&self, database: &mut Database, foreign_id: u64) -> Result<Row, String> {
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

    pub fn get(&self, table_name: String, database: &mut Database) -> Result<Row, String> {
        let relation: Relation = {
            // Get table
            let table: &mut Table = database.get_table(table_name).unwrap();

            // Fetch the Relation
            table.get_relation(&self.relation_name).unwrap().clone()
        };
        // Fetch row from foreign table
        let foreign_row = relation.get_foreign_row(database, self.foreign_id)?;
        // Now you can continue using 'relation' or anything else

        Ok(foreign_row)
    }

    pub fn from_value(value: Value) -> OneToOne {
        serde_json::from_value(value).unwrap()
    }
}
