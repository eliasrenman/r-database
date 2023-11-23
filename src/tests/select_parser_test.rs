#[cfg(test)]
mod select_parser_test {
    use crate::database::{
       query::select_parser::SelectParser
    };
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    #[test]
    fn should_parse_selector_recursive() {
        let output =
            SelectParser::parse_selector_recursive(vec!["id", "food", "relation.*", "relation.food.*"]);
        assert_json_include!(
            actual: &output,
            expected: &json!({
             "id": "id",
             "food": "food",
             "relation": {
               "*": "*",
               "food": {
                 "*": "*"
               }
             }
             })
        );
    }
    #[test]
    fn should_parse_selector_recursive_one_level() {
        let output = SelectParser::parse_selector_recursive(vec!["id", "name", "foods.*"]);
        let json = serde_json::to_string_pretty(&output).unwrap();
        println!("{json}");
        assert_json_include!(
            actual: &output,
            expected: &json!({
             "id": "id",
             "name":"name",
             "foods": {
               "*": "*",
             }
             })
        );
    }
}
