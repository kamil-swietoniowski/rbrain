use rusqlite::Connection;

pub struct Database {
    pub db: Connection,
}
