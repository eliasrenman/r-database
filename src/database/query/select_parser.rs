use serde_json::{Map, Value, json};

pub struct SelectParser {}
impl SelectParser {
  pub fn parse_selector_recursive(node: Vec<&str>) -> Value {
    let mut output = Map::new();

    for key in node {
      if let Some(dot_index) = key.find('.') {
        let next_key = key[..dot_index].to_string();
        let next_node = key[dot_index + 1..].to_string();

        let mut nested_value = SelectParser::parse_selector_recursive(vec![&next_node]);

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