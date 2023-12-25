use crate::database::{self, Database};

use super::{select_parser::SelectParser, select_processor::SelectProcessor};

pub struct QueryBuilder<'a> {
    database: &'a Database,
}

impl<'a> QueryBuilder<'a> {
    fn new(database: &'a Database) -> QueryBuilder {
        QueryBuilder { database }
    }

    fn select(self) {
        // select query builder
        // where query builder
        // Combine them
        let output = SelectProcessor::selector(self.database, &cat_table.name, cat_1, select);
    }
}
