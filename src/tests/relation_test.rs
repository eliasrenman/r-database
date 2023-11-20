#[cfg(test)]
mod relation_test {
    use std::borrow::BorrowMut;

    use crate::database::{
        relation::{OneToOne, Relation, OneToMany},
        table::Table,
        Database,
    };

    #[test]
    fn should_succed_one_to_one_relationship_found() {
        let cat_relations =
            hashmap!["cat_food" => Relation::new("Foods".to_string(), "one_to_one".to_string())];
        let cat_table: Table = Table::new("Cats", "id", None, Some(cat_relations));

        let food_table: Table = Table::new("Foods", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        _ = database
            .get_table_mut("Foods".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Dry Feed"]);
        let _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table_mut("Cats".to_string()).unwrap();
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
    fn should_fail_no_one_to_one_relation_found() {
        let cat_table: Table = Table::new("Cats", "id", None, None);

        let food_table: Table = Table::new("Foods", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        _ = database
            .get_table_mut("Foods".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Dry Feed"]);
        let _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table_mut("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Foods".to_string(), database.borrow_mut());

        assert_eq!(foreign_row.is_err(), true);
    }

    #[test]
    fn should_fail_no_table_found() {
      let cat_table: Table = Table::new("Cats", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table]);

        let _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table_mut("Cats".to_string()).unwrap();
        let cat_1 = cat_table.find_by_pk(1u64).unwrap();

        let cat_1_food = cat_1.get("food").unwrap();

        let relation = OneToOne::from_value(cat_1_food.to_owned());
        assert_eq!(relation.get_id(), 123u64);

        let foreign_row = relation.get("Foods".to_string(), database.borrow_mut());
        print!(
            "Printing foreign row error {}\n",
            &foreign_row.unwrap_err().as_str()
        );
    }
    #[test]
    fn should_fail_no_one_to_one_row_found() {
        let cat_table: Table = Table::new("Cats", "id", None, None);

        let food_table: Table = Table::new("Foodss", "id", None, None);

        let mut database: Database = Database::new(vec![cat_table, food_table]);

        let _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "food" => OneToOne::new(123u64, "cat_food".to_string())]);

        let cat_table = database.get_table_mut("Cats".to_string()).unwrap();
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
    #[test]
    fn should_succed_one_to_many_relationship_found() {
      let cat_relations =
            hashmap!["cat_food" => Relation::new("Foods".to_string(), "one_to_many".to_string())];
      let cat_table: Table = Table::new("Cats", "id", None, Some(cat_relations));

      let food_table: Table = Table::new("Foods", "id", None, None);

      let mut database: Database = Database::new(vec![cat_table, food_table]);

      _ = database
            .get_table_mut("Foods".to_string())
            .unwrap()
            .insert_row(row!["id" => 123, "name" => "Dry Feed"]);
      _ = database
            .get_table_mut("Foods".to_string())
            .unwrap()
            .insert_row(row!["id" => 12, "name" => "Dry Feed"]);
      let _ = database.get_table_mut("Cats".to_string()).unwrap().insert_row(row!["id" => 1, "name" => "Ozzy", "breed" => "mixed", "foods" => OneToMany::new(vec![123u64,12u64], "cat_food".to_string())]);

      let cat_table = database.get_table("Cats".to_string()).unwrap();
      let cat_1 = cat_table.find_by_pk(1u64).unwrap();

      let cat_1_food = cat_1.get("foods").unwrap();

      let relation = OneToMany::from_value(cat_1_food.to_owned());
      assert_eq!(relation.get_ids(), vec![123u64, 12u64]);

      let foreign_row = relation.get("Cats".to_string(), database.borrow_mut());

      assert_eq!(foreign_row.is_ok(), true);

      let foreign_rows_unwrapped = foreign_row.unwrap();

      assert_eq!(foreign_rows_unwrapped.len(), 2);
      for row in foreign_rows_unwrapped.into_iter() {
        print!(
          "Printing foreign row {}\n",
            serde_json::to_string(&row).unwrap()
        );
      };

    }
}
