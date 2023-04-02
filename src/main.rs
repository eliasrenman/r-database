use serde_json::Value;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map: ::std::collections::HashMap<String, Value> = ::std::collections::HashMap::new();
         $( map.insert($key.to_string(), serde_json::to_value($val).unwrap()); )*
         map
    }}
}
use database::{table::Table, row::Row};

mod database;



fn main() {
    let mut table: Table = Table::new("Cats", "id");
    table.insert_row(Row::new(hashmap!["id" => 1, "name" => "Ozzy"]));
    table.insert_row(Row::new(hashmap!["id" => 2, "name" => "Simon"]));

    let row = table.find_by_pk(&1u64);
    let pretty_print = serde_json::to_string_pretty(row);
    
    println!("Found Row: {}", pretty_print.unwrap());
    let row = table.find_by_pk(&2u64);
    let pretty_print = serde_json::to_string_pretty(row);
    
    println!("Found Row: {}", pretty_print.unwrap());
    let row = table.find_by_pk(&3u64);
    let pretty_print = serde_json::to_string_pretty(row);
    
    println!("Failed to Find Row: {}", pretty_print.unwrap());

}
