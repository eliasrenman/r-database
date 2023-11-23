//hashmap!(["food" => Relation::new("food","one_to_one")])

#[cfg(test)]
mod select_processor_test {
    use crate::database::{
        query::select_processor::SelectProcessor,
        relation::{OneToMany, OneToOne, Relation},
        table::Table,
        Database,
    };
    use assert_json_diff::assert_json_include;
    use serde_json::json;

    pub fn initialize() -> Database {
        let cat_relations = hashmap!["cat_food" => Relation::new("Food".to_string(), "one_to_many".to_string()),
            "cat_age_group" => Relation::new("AgeGroup".to_string(), "one_to_one".to_string())];
        let food_relations = hashmap!["food_ingredient" => Relation::new("Ingredient".to_string(), "one_to_many".to_string())];
        let cat_table: Table = Table::new("Cats", "id", None, Some(cat_relations));

        let food_table: Table = Table::new("Food", "id", None, Some(food_relations));
        let ingredient_table: Table = Table::new("Ingredient", "id", None, None);
        let age_table: Table = Table::new("AgeGroup", "id", None, None);

        let mut database: Database =
            Database::new(vec![cat_table, food_table, age_table, ingredient_table]);

        _ = database
            .get_table_mut("Food".to_string())
            .unwrap()
            .insert_row(row![
            "id" => 123,
            "name" => "Wet Feed",
            "ingridients" => OneToMany::new(vec![], "food_ingredient".to_string())]);
        _ = database
            .get_table_mut("Food".to_string())
            .unwrap()
            .insert_row(row![
            "id" => 12,
            "name" => "Dry Feed",
            "ingridients" => OneToMany::new(vec![33u64], "food_ingredient".to_string())
            ]);
        _ = database
            .get_table_mut("Ingredient".to_string())
            .unwrap()
            .insert_row(row!["id" => 33, "name" => "Chicken Meat"]);
        _ = database
            .get_table_mut("AgeGroup".to_string())
            .unwrap()
            .insert_row(row!["id" => 99, "age" => "Young"]);
        _ = database
            .get_table_mut("Cats".to_string())
            .unwrap()
            .insert_row(row![
        "id" => 1,
        "name" => "Ozzy",
        "breed" => "mixed",
        "foods" => OneToMany::new(vec![123u64,12u64], "cat_food".to_string()),
        "group" => OneToOne::new(99,"cat_age_group".to_string())]);

        return database;
    }

    #[test]
    fn should_succed_select_processor_two_level_resolved() {
        let database = initialize();

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();
        let select = vec!["id", "name", "foods.*", "foods.ingridients.name"];
        let output = SelectProcessor::selector(&database, &cat_table.name, cat_1, select);
        let pretty = serde_json::to_string_pretty(&output).unwrap();
        println!("Json output {pretty}");
        assert_json_include!(
            actual: &output,
            expected:
                &row!["id" => 1, "name" =>"Ozzy", "foods" =>
                 vec![
                   row!["id" => 123, "name" => "Wet Feed", "ingridients" => json!([])],
                   row!["id" => 12, "name" => "Dry Feed", "ingridients" => vec![row!["name" => "Chicken Meat"]]]
                 ]]
        );
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
    fn should_succed_select_processor_one_level_resolved_onetoone() {
        let database = initialize();

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();
        let select = vec!["id", "name", "group.age"];
        let output = SelectProcessor::selector(&database, &cat_table.name, cat_1, select);
        assert_json_include!(
            actual: &output,
            expected:
                &row!["id" => 1, "name" =>"Ozzy", "group"=> row![
                   "age" => "Young"
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
}
