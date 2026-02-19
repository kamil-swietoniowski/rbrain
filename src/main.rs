use std::path::Path;

use rbrain::api::{data_models::database_model::Record, database::databasesqlite::Database};

pub mod api;

fn main() {
    let db = Database::new(Path::new("baza_danych.db")).unwrap();
    let record = Record::new("Test note num 2", "Very testy second test, not suspicous test note");
    db.insert_record_to_database(record);
    println!("Hello, world!");
}
