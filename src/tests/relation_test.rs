#[cfg(test)]
mod relation_test {
    use std::borrow::BorrowMut;

    use crate::database::{
        relation::{OneToOne, Relation},
        table::Table,
        Database,
    };

    #[test]
    fn should_succed_relationship_found() {
        let cat_relations =
            hashmap!["cat_food" => Relation::new("Food".to_string(), "one_to_one".to_string())];
        let cat_table: Table = Table::new("Cats", "id", None, Some(cat_relations));

        let food_table: Table = Table::new("Food", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        _ = database
            .get_table("Food".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Dry Feed"]);
        let _ = database.get_table("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Cats".to_string(), database.borrow_mut());

        assert_eq!(foreign_row.is_ok(), true);

        let foreign_row_unwrapped = foreign_row.unwrap();

        assert_eq!(
            foreign_row_unwrapped.get("id").unwrap().as_u64().unwrap(),
            123u64
        );
        print!(
            "Printing foreign row {}\n",
            serde_json::to_string(&foreign_row_unwrapped).unwrap()
        );
    }

    #[test]
    fn should_fail_no_relation_found() {
        let cat_table: Table = Table::new("Cats", "id", None, None);

        let food_table: Table = Table::new("Food", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        _ = database
            .get_table("Food".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Dry Feed"]);
        let _ = database.get_table("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Food".to_string(), database.borrow_mut());

        assert_eq!(foreign_row.is_err(), true);
    }

    #[test]
    fn should_fail_no_table_found() {
        let cat_table: Table = Table::new("Cats", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table]);

        let _ = database.get_table("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Food".to_string(), database.borrow_mut());
        print!(
            "Printing foreign row error {}\n",
            &foreign_row.unwrap_err().as_str()
        );
    }
    #[test]
    fn should_fail_no_row_found() {
        let cat_table: Table = Table::new("Cats", "id", None, None);

        let food_table: Table = Table::new("Food", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        let _ = database.get_table("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Cats".to_string(), database.borrow_mut());
        print!(
            "Printing foreign row error {}\n",
            &foreign_row.unwrap_err().as_str()
        );
    }
    // Fail without table found, invalid relation
    // Fail row and table found but not row
}
