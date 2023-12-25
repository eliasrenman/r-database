use crate::database::{self, Database};

pub struct WhereFetcher {}

impl WhereFetcher {
    pub fn find_rows(database: &Database, table_name: String, where_query: String) {}
}
