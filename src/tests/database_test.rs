#[cfg(test)]
mod database_test {
    use crate::database::{table::Table, Database};
    use std::{fs, path::Path};

    #[test]
    fn should_write_and_read_to_file() {
        let mut table: Table = Table::new("Cats", "id", None, None);
        let _ = table.insert_row(row!["id" => 1, "name" => "Ozzy"]);
        _ = table.insert_row(row!["id" => 2, "name" => "Simon"]);
        let database = Database::new(vec![table]);

        database.to_file("./db.json");

        let exists = Path::try_exists(Path::new("./db.json"));
        assert_eq!(exists.unwrap(), true);

        let table = Database::from_file("./db.json".to_owned());
        assert_eq!(table.is_ok(), true);

        let mut db = table.unwrap();
        let table = db.get_table("Cats".to_string());
        if table.is_err() {
            panic!("Unable to find table");
        }
        let table = table.unwrap();
        let row = table.find_by_pk(1u64);
        assert_eq!(row.is_ok(), true);

        // Cleanup file
        let _result = fs::remove_file("./db.json");
    }
}
