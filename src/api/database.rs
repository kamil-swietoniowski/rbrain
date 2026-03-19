use std::fmt::Result;

use rusqlite::{Connection};

const SCHEMA: &str = include_str!("./schema.sql");



fn init_db(path: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

pub struct Database {
    conn: Connection
}

impl Database {
    pub fn new(path: &str) -> Self {
        let conn = init_db(path).unwrap_or_else(|error| {
            eprintln!("Error occured: {}", error);
            std::process::exit(0);
        });
        Self { conn }
    }

}
