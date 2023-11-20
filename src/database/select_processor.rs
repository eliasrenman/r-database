use std::{borrow::Borrow, collections::HashMap, str::FromStr};

use serde_json::{map::Values, Value, json};

use crate::database::relation::{OneToOne, OneToMany};

use super::{row::Row, table::Table, Database};

pub struct SelectProcessor {
  
}
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
  
  pub fn selector(database: &Database, table_name: &String, row: &Row, select: Vec<String>, mut output: HashMap<String,Value>) -> HashMap<String,Value> {

    for value in select {
      let key = value.to_string();
      if(key.eq("*")) {
        return row.clone();// recursive call
      }
      else if(key.contains(".")) {
        let split: Vec<String> = key.split(".").map(|part| String::from_str(part).unwrap()).collect();
        
        if(split.len() == 2 && split[1].eq("*")) {
          let relation_rows = SelectProcessor::resolve_relation(database, table_name, row, &split[0]);
          output.insert(split[0].clone(), json!(relation_rows));
        }
        // recursive call
      } else {
        let value = row.get(&key);
        match value.is_some() {
          true =>  output.insert(key, value.unwrap().to_owned()),
          false => panic!("Failed to read key: {key} on table: {table_name}"),
          
        };
          
      }
    }
    return output;
  }
  
  pub fn resolve_relation(database: &Database, table_name: &String, row: &Row, key: &String) -> Vec<Row> {
    let table = database.get_table(table_name.clone()).unwrap();
    
    println!("Attempting to find relation for '{key}' in table: '{table_name}'");
    
    
    let value = row.get(key).unwrap();
    let relation_name = match value.get("relation_name") {
      Some(val) => String::from(val.as_str().unwrap()),
      None => panic!("Value was not a valid relation")
    };
    let relation = table.get_relation(&relation_name).unwrap();
    let relation_variation = relation.variation.as_str();
    return match relation_variation {
      "one_to_one" => vec![OneToOne::from_value(value.to_owned()).get(table_name.clone(), database).unwrap()],
      "one_to_many" => OneToMany::from_value(value.to_owned()).get(table_name.clone(), database).unwrap(),
      other => panic!("Unsupported relationship type: {other}")
    }
  }
}