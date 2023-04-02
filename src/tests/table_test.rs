#[cfg(test)]
mod tests {

    use crate::database::{table::Table, row::Row};

    #[test]
    fn should_find_two_rows() {
        let mut table: Table = Table::new("Cats", "id");
        table.insert_row(Row::new(hashmapJson!["id" => 1, "name" => "Ozzy"]));
        table.insert_row(Row::new(hashmapJson!["id" => 2, "name" => "Simon"]));

        let row = table.find_by_pk(&1u64);
        assert_eq!(row.is_err(), false);
        let pretty_print = serde_json::to_string_pretty(row.unwrap());

        println!("Found Row: {}", pretty_print.unwrap());
        let row = table.find_by_pk(&2u64);
        let pretty_print = serde_json::to_string_pretty(row.unwrap());

        println!("Found Row: {}", pretty_print.unwrap());
    }

    #[test]
    fn should_fail_to_find_row() {
        let mut table: Table = Table::new("Cats", "id");
        table.insert_row(Row::new(hashmapJson!["id" => 1, "name" => "Ozzy"]));
        table.insert_row(Row::new(hashmapJson!["id" => 2, "name" => "Simon"]));

        let row = table.find_by_pk(&3u64);
        assert_eq!(row.is_err(), true);
    }


}
