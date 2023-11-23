use serde_json::{json, Map, Value};

use crate::database::{
    query::select_parser::SelectParser,
    relation::{OneToMany, OneToOne},
    row::Row,
    Database,
};

pub struct SelectProcessor {}

impl SelectProcessor {
    // recursively select relations

    pub fn selector(
        database: &Database,
        table_name: &String,
        row: &Row,
        select: Vec<&str>,
    ) -> Map<String, Value> {
        let selector = SelectParser::parse_selector_recursive(select);
        SelectProcessor::recursive_traverse_resolver(database, table_name, row, &selector)
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
            match value {
                Value::Object(_val) => {
                    output.insert(
                        key.to_owned(),
                        SelectProcessor::resolve_foreign_relation(
                            database, table_name, row, key, value,
                        ),
                    );
                }
                Value::String(val) => {
                    if val == "*" {
                        output = row.to_owned();
                        continue;
                    }
                    let val = row.get(key);
                    if val.is_none() {
                        let pretty_json = serde_json::to_string_pretty(row).unwrap();
                        panic!(
                            "Value missing on key {key} in table {table_name} - row: {pretty_json}"
                        );
                    }
                    output.insert(key.to_owned(), json!(val.unwrap()));
                }
                _ => panic!("Value has invalid type"),
            }
        }
        return output;
    }

    fn resolve_foreign_relation(
        database: &Database,
        table_name: &String,
        row: &Row,
        key: &String,
        selector: &Value,
    ) -> Value {
        let (new_table_name,relation_row_value) =
            SelectProcessor::resolve_relation(database, table_name, row, key);

        if relation_row_value.is_array() {
            let mut row_vec: Vec<Map<String, Value>> = vec![];
            for row in relation_row_value.as_array().unwrap() {
                let asd = row.as_object().unwrap();
                let parsed_row = SelectProcessor::recursive_traverse_resolver(
                  database, &new_table_name, &asd, &selector,
                );
                row_vec.push(parsed_row);
            }
            return json!(row_vec);
        }

        let foreign_row = relation_row_value.as_object().unwrap();
        let parsed_row = SelectProcessor::recursive_traverse_resolver(
            database,
            table_name,
            foreign_row,
            &selector,
        );
        return json!(parsed_row);
    }

    pub fn resolve_relation(
        database: &Database,
        table_name: &String,
        row: &Row,
        key: &String,
        ) -> (String,Value) {
        let table = database.get_table(table_name.clone()).unwrap();

        println!("Attempting to find relation for '{key}' in table: '{table_name}'");
        let value = row.get(key).unwrap();
        let relation_name = match value.get("relation_name") {
            Some(val) => String::from(val.as_str().unwrap()),
            None => panic!("Value was not a valid relation"),
        };
        let relation = table.get_relation(&relation_name).unwrap();
        let relation_value = match relation.variation.as_str() {
            "one_to_one" => json!(OneToOne::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap()),
            "one_to_many" => json!(OneToMany::from_value(value.to_owned())
                .get(table_name.clone(), database)
                .unwrap()),
            other => panic!("Unsupported relationship type: {other}"),
        };
        (relation.table_name.to_owned(), relation_value)
    }

}
