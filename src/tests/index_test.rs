#[cfg(test)]
mod index_test {

    use serde_json::json;

    use crate::database::{index::Index, table::Table};

    #[test]
    fn should_create_string_index_and_find_two_results() {
        let indexes = hashmap!["race" => Index::new("race".to_string(), Some(false), Some(false))];
        let mut table: Table = Table::new("Cats", "id", Some(indexes));
        let _ = table.insert_row(row!["id" => 1, "name" => "Ozzy", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 2, "name" => "Simon", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 3, "name" => "Gosa", "race" => "dog"]);

        let result = table.get_pks_by_index("race".to_string(), json!("cat"));

        assert_eq!(result.is_ok(), true);

        let ids = result.unwrap();
        assert_eq!(ids, vec![1, 2]);

        let rows = table.find_by_pks(ids.clone());

        assert_eq!(rows.len(), 2);
        for row in rows {
            assert_eq!(row.is_ok(), true);
            let pretty_print = serde_json::to_string_pretty(row.unwrap());
            println!("Found Row: {}", pretty_print.unwrap());
        }
    }

    #[test]
    fn should_create_reverse_index() {
        let indexes = hashmap!["race" => Index::new("race".to_string(), Some(true), Some(false))];
        let mut table: Table = Table::new("Cats", "id", Some(indexes));
        let _ = table.insert_row(row!["id" => 1, "name" => "Ozzy", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 2, "name" => "Simon", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 3, "name" => "Gosa", "race" => "dog"]);

        let result = table.get_pks_by_index("race".to_string(), json!("cat"));

        assert_eq!(result.is_ok(), true);

        let ids = result.unwrap();
        assert_eq!(ids, vec![2, 1]);
    }

    #[test]
    fn should_create_unique_index_and_fail_insert() {
        let indexes = hashmap!["race" => Index::new("race".to_string(), Some(false), Some(true))];
        let mut table: Table = Table::new("Cats", "id", Some(indexes));
        let _ = table.insert_row(row!["id" => 1, "name" => "Ozzy", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 3, "name" => "Gosa", "race" => "dog"]);

        let err = table.insert_row(row!["id" => 2, "name" => "Simon", "race" => "cat"]);
        assert_eq!(err.is_err(), true);
    }

    #[test]
    fn should_create_unique_index() {
        let indexes = hashmap!["race" => Index::new("race".to_string(), Some(false), Some(true))];
        let mut table: Table = Table::new("Cats", "id", Some(indexes));
        let _ = table.insert_row(row!["id" => 1, "name" => "Ozzy", "race" => "cat"]);
        _ = table.insert_row(row!["id" => 3, "name" => "Gosa", "race" => "dog"]);

        let result = table.get_pks_by_index("race".to_string(), json!("cat"));

        assert_eq!(result.is_ok(), true);

        let ids = result.unwrap();
        assert_eq!(ids, vec![1]);

        let result = table.get_pks_by_index("race".to_string(), json!("dog"));

        assert_eq!(result.is_ok(), true);

        let ids = result.unwrap();
        assert_eq!(ids, vec![3]);
    }
}
