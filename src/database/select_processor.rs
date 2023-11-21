use std::{borrow::Borrow, collections::HashMap, str::FromStr};

use serde_json::{json, map::Values, Map, Value};

use crate::database::relation::{OneToMany, OneToOne};

use super::{row::Row, table::Table, Database};

pub struct SelectProcessor {}
/**
{
  select: [
    "id",
    "name",
    "relation.*"
  ]
}
{
  select: [
    "id",
    "name",
    "relation.id"
  ]
}
//
*/
impl SelectProcessor {
    // recursively select relations

    pub fn selector(
        database: &Database,
        table_name: &String,
        row: &Row,
        select: Vec<String>,
        mut output: HashMap<String, Value>,
    ) -> HashMap<String, Value> {
        for value in select {
            let key = value.to_string();
            if (key.eq("*")) {
                return row.clone(); // recursive call
            } else if (key.contains(".")) {
                let split: Vec<String> = key
                    .split(".")
                    .map(|part| String::from_str(part).unwrap())
                    .collect();

                if (split.len() == 2 && split[1].eq("*")) {
                    let relation_rows =
                        SelectProcessor::resolve_relation(database, table_name, row, &split[0]);
                    output.insert(split[0].clone(), json!(relation_rows));
                }
                // recursive call
            } else {
                let value = row.get(&key);
                match value.is_some() {
                    true => output.insert(key, value.unwrap().to_owned()),
                    false => panic!("Failed to read key: {key} on table: {table_name}"),
                };
            }
        }
        return output;
    }

    pub fn resolve_relation(
        database: &Database,
        table_name: &String,
        row: &Row,
        key: &String,
    ) -> Vec<Row> {
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
            "one_to_one" => vec![OneToOne::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap()],
            "one_to_many" => OneToMany::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap(),
            other => panic!("Unsupported relationship type: {other}"),
        };
    }

    fn is_valid_key(key: &str) -> bool {
        key.chars().all(|c| c.is_ascii_lowercase() || c == '_')
    }

    pub fn recursive_parse_select(node: Vec<&str>) -> Value {
        let mut output = Map::new();

        for key in node {
            if let Some(dot_index) = key.find('.') {
                let next_key = key[..dot_index].to_string();
                let next_node = key[dot_index + 1..].to_string();

                let mut nested_value = SelectProcessor::recursive_parse_select(vec![&next_node]);

                if output.contains_key(&next_key) {
                    let deep_map = output.get_mut(&next_key).unwrap().as_object_mut().unwrap();
                    deep_map.append(nested_value.as_object_mut().unwrap());
                } else {
                    output.insert(next_key, nested_value);
                }
                output.insert(key.to_string(), json!(key));
            }
        }

        json!(output)
    }
}
