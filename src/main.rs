
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map: ::std::collections::HashMap<String, ::serde_json::Value> = ::std::collections::HashMap::new();
         $( map.insert($key.to_string(), serde_json::to_value($val).unwrap()); )*
         map
    }}
}

mod database;

fn main() {
}

#[cfg(test)]
mod tests;