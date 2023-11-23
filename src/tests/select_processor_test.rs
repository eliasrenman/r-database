//hashmap!(["food" => Relation::new("food","one_to_one")])

#[cfg(test)]
mod select_processor_test {
    use crate::database::{
        relation::{OneToMany, Relation},
        select_processor::SelectProcessor,
        table::Table,
        Database,
    };
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    pub fn initialize() -> Database {
        let cat_relations =
            hashmap!["cat_food" => Relation::new("Food".to_string(), "one_to_many".to_string())];
        let cat_table: Table = Table::new("Cats", "id", None, Some(cat_relations));

        let food_table: Table = Table::new("Food", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        _ = database
            .get_table_mut("Food".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Wet Feed"]);
        _ = database
            .get_table_mut("Food".to_string())
            .unwrap()
            .insert_row(row!["id" => 12, "name" => "Dry Feed"]);
        _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "foods" => OneToMany::new(vec![123u64,12u64], "cat_food".to_string())]);

        return database;
    }
    #[test]
    fn should_succed_select_processor_one_level_resolved() {
        let database = initialize();

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();
        let select = vec!["id", "name", "foods.*"];
        let output = SelectProcessor::selector(&database, &cat_table.name, cat_1, select);
        assert_json_include!(
            actual: &output,
            expected:
                &row!["id" => 1, "name" =>"Ozzy", "foods" =>
                 vec![
                   row!["id" => 123, "name" => "Wet Feed"],
                   row!["id" => 12, "name" => "Dry Feed"]

                 ]]
        );
    }
    #[test]
    fn should_succed_select_processor_zero_level_resolved() {
        let database = initialize();

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();
        let select = vec!["id", "name", "breed"];
        let output = SelectProcessor::selector(&database, &cat_table.name, cat_1, select);
        assert_json_include!(
            actual: &output,
            expected: &row!["id" => 1, "name" =>"Ozzy", "breed" => "mixed"]
        );
    }
    #[test]
    fn should_succed_select_processor_zero_level_asterix_resolved() {
        let database = initialize();

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();
        let select = vec!["*"];
        let output = SelectProcessor::selector(&database, &cat_table.name, cat_1, select);

        assert_json_include!(
            actual: &output,
            expected:
                &json!({"id": 1, "name":"Ozzy", "breed": "mixed", "foods":
                   {
                     "relation_name": "cat_food",
                     "foreign_ids": [123,12]
                   }
                 })
        );
    }

    #[test]
    fn should_parse_node() {
        let output =
            SelectProcessor::parse_node(vec!["id", "food", "relation.*", "relation.food.*"]);
        let json = serde_json::to_string_pretty(&output).unwrap();
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
    fn should_parse_node_one_level() {
        let output = SelectProcessor::parse_node(vec!["id", "name", "foods.*"]);
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
