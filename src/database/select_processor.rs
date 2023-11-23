use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::database::relation::{OneToMany, OneToOne};

use super::{row::Row, Database};

pub struct SelectProcessor {}

impl SelectProcessor {
    // recursively select relations

    pub fn selector(
        database: &Database,
        table_name: &String,
        row: &Row,
        select: Vec<&str>,
    ) -> HashMap<String, Value> {
        let asd = SelectProcessor::parse_node(select);
        println!("Debugging parsed selector {asd}");
        SelectProcessor::recursive_traverse_resolver(database, table_name, row, &asd)
    }

    fn recursive_traverse_resolver(
        database: &Database,
        table_name: &String,
        row: &Row,
        selector: &Value,
    ) -> HashMap<String, Value> {
        let object = selector.as_object().unwrap();
        let mut output: HashMap<String, Value> = HashMap::new();
        for (key, value) in object.into_iter() {
            if value.is_object() {
                let relation_rows: Value =
                    SelectProcessor::resolve_relation(database, table_name, row, key);

                if relation_rows.is_array() {
                  let mut row_vec: Vec<HashMap<String, Value>> = vec![];
                  for row in relation_rows.as_array().unwrap() {
                    let asd = Row::from(row.as_object().unwrap());
                    let parsed_row = SelectProcessor::recursive_traverse_resolver(
                      database, table_name, &(row.as_object().unwrap()), &value,
                        );
                        row_vec.push(parsed_row);
                    }
                    output.insert(key.to_owned(), json!(row_vec));
                } else {
                    let parsed_row = SelectProcessor::recursive_traverse_resolver(
                        database, table_name, &row, &value,
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

    pub fn parse_node(node: Vec<&str>) -> Value {
        let mut output = Map::new();

        for key in node {
            if let Some(dot_index) = key.find('.') {
                let next_key = key[..dot_index].to_string();
                let next_node = key[dot_index + 1..].to_string();

                let mut nested_value = SelectProcessor::parse_node(vec![&next_node]);

                if output.contains_key(&next_key) {
                    let current_value = output.get_mut(&next_key).unwrap().as_object_mut().unwrap();
                    current_value.append(nested_value.as_object_mut().unwrap());
                } else {
                    output.insert(next_key, nested_value);
                }
            } else {
                output.insert(key.to_string(), json!(key));
            }
        }

        json!(output)
    }
}
