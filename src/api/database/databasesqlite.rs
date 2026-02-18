use std::path::Path;

use rusqlite::{Connection, Result};

const SCHEMA: &str = include_str!("./schema.sql");

pub struct Database {
    pub db: Connection,
}

impl Database {
    pub fn new(file_path: &Path) -> Result<Database> {
        let db = Connection::open(file_path)?;
        db.execute_batch(SCHEMA)?;
        Ok(Database { db } )
    }
}
