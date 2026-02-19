use std::path::Path;

use rusqlite::{Connection, Result};
use crate::api::{data_models::database_model::Record, utils::util_tools::{self, now}};

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
     
    pub fn insert_record_to_database(&self, record: Record) -> rusqlite::Result<()> {
        let title = record.title;
        let content = if let Some(content) = record.content {content} else {"".into()};
        let created_at = record.created_at;
        let updated_at = record.updated_at;
     
        self.db.execute("INSERT INTO record (title, content, modified_at, created_at) VALUES(?1, ?2, ?3, ?4)", [title, content, updated_at, created_at])?;
        Ok(())
    }
     
    pub fn update_record_in_database(&self, id: i32, title: Option<String>, content: Option<String>) -> rusqlite::Result<()> {
        let id = id.to_string();
        let now = util_tools::now();
        match (title, content) {
            (Some(title), None) => self.db.execute("UPDATE record SET title = ?1, modified_at = ?2 WHERE ID = ?3", [title, now, id])?,
            (None, Some(content)) => self.db.execute("UPDATE record SET content = ?1, modified_at = ?2 WHERE ID = ?3", [content, now, id])?,
            (Some(title), Some(content)) => self.db.execute("UPDATE record SET title = ?1, content = ?2, modified_at = ?3 WHERE ID = ?4", [title, content, now, id])?,
            _ => 0
        };
        Ok(())
    }
     
    pub fn get_record_from_database(&self, id: i32) -> rusqlite::Result<Record> {
        todo!();
    }

    pub fn get_list_of_records_from_database(&self) -> rusqlite::Result<Vec<Record>> {
        todo!();
    } 
     
    pub fn insert_tag_to_database(&self, tag: String) -> rusqlite::Result<()> {
        todo!();
    }
     
    pub fn remove_record_from_database(&self, id: i32) -> rusqlite::Result<()> {
        todo!();
    }
}
