use serde_json::{json, Map, Value};

use crate::database::{relation::{OneToMany, OneToOne}, Database, row::Row, query::select_parser::SelectParser};


pub struct SelectProcessor {}

impl SelectProcessor {
    // recursively select relations

    pub fn selector(
        database: &Database,
        table_name: &String,
        row: &Row,
        select: Vec<&str>,
    ) -> Map<String, Value> {
        let asd = SelectParser::parse_selector_recursive(select);
        println!("Debugging parsed selector {asd}");
        SelectProcessor::recursive_traverse_resolver(database, table_name, row, &asd)
    }

    fn recursive_traverse_resolver(
        database: &Database,
        table_name: &String,
        row: &Row,
        selector: &Value,
    ) -> Map<String, Value> {
        let object = selector.as_object().unwrap();
        let mut output: Map<String, Value> = Map::new();
        for (key, value) in object.into_iter() {
            if value.is_object() {
                let relation_rows: Value =
                    SelectProcessor::resolve_relation(database, table_name, row, key);

                if relation_rows.is_array() {
                    let mut row_vec: Vec<Map<String, Value>> = vec![];
                    for row in relation_rows.as_array().unwrap() {
                        let asd = row.as_object().unwrap();
                        let parsed_row = SelectProcessor::recursive_traverse_resolver(
                            database, table_name, &asd, &value,
                        );
                        row_vec.push(parsed_row);
                    }
                    output.insert(key.to_owned(), json!(row_vec));
                } else {
                  let foreign_row = relation_rows.as_object().unwrap();
                    let parsed_row = SelectProcessor::recursive_traverse_resolver(
                      database, table_name, foreign_row, &value,
                    );

                    output.insert(key.to_owned(), json!(parsed_row));
                }
            } else {
                if value.as_str().unwrap() == "*" {
                    output = row.to_owned();
                } else {
                    let val = row.get(key);
                    if val.is_none() {
                        let pretty_json = serde_json::to_string_pretty(row).unwrap();
                        panic!(
                            "Value missing on key {key} in table {table_name} - row: {pretty_json}"
                        );
                    }
                    output.insert(key.to_owned(), json!(val.unwrap()));
                }
            }
        }
        return output;
    }

    pub fn resolve_relation(
        database: &Database,
        table_name: &String,
        row: &Row,
        key: &String,
    ) -> Value {
        let table = database.get_table(table_name.clone()).unwrap();

        println!("Attempting to find relation for '{key}' in table: '{table_name}'");

        let value = row.get(key).unwrap();
        let relation_name = match value.get("relation_name") {
            Some(val) => String::from(val.as_str().unwrap()),
            None => panic!("Value was not a valid relation"),
        };
        let relation = table.get_relation(&relation_name).unwrap();
        let relation_variation = relation.variation.as_str();
        return match relation_variation {
            "one_to_one" => json!(OneToOne::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap()),
            "one_to_many" => json!(OneToMany::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap()),
            other => panic!("Unsupported relationship type: {other}"),
        };
    }

    fn is_valid_key(key: &str) -> bool {
        key.chars().all(|c| c.is_ascii_lowercase() || c == '_')
    }
}
