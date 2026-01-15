use std::collections::HashMap;
use chrono::{DateTime, offset::Utc};
use rusqlite::{Connection, Result, OptionalExtension};

const SCHEMA: &str = include_str!("../schema.sql");

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;
    conn.execute_batch(SCHEMA)?;
    Ok(conn)
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Self{
        let conn = init_db(path).unwrap_or_else(|error|{
            eprintln!("Error occured: {}", error);
            std::process::exit(0);
        });
        Self { conn }
    }
    pub fn insert_record_to_database(&self, record: &Record) -> rusqlite::Result<()>{
        let title = match &record.title {
            Some(title) => title,
            None => &"".to_string(),
        };
        let content = match &record.content {
            Some(content) => content.as_bytes().to_vec(),
            None => Vec::new(),
        };
        let modified_at = match &record.modified_at {
            Some(modified) => modified.to_rfc3339(),
            None => "".to_string(),
        };
        let created_at = match &record.created_at {
            Some(created) => created.to_rfc3339(),
            None => "".to_string(),
        };

        self.conn.execute(
            "INSERT INTO record (title, content, modified_at, created_at) VALUES (?1, ?2, ?3, ?4)",
            (title, &content, modified_at, created_at),
        )?;
        Ok(())
    }

    pub fn modify_record(&self, id: i32, title: Option<&str>, content: Option<&str>) -> rusqlite::Result<()> {
        let modified_at = Utc::now().to_rfc3339();

        self.conn.execute(
            "UPDATE record SET title = ?1, content = ?2, modified_at = ?3 WHERE id = ?4" ,
            (title, content, modified_at, id),
        )?;
        Ok(())
    }



    pub fn get_record_from_database(&self, id: i32) -> rusqlite::Result<Record> {
        self.conn.query_row(
            "SELECT id, title, content, modified_at, created_at FROM record WHERE id = ?1", [id], |row| {
                Ok(Record{
                    id: {
                        let i: String = row.get(0)?;
                        i.parse::<i32>().unwrap()

                    },
                    title: row.get(1)?,
                    content: row.get(2)?,
                    modified_at: {
                        let s: String = row.get(3)?;
                        Some(s.parse::<DateTime<Utc>>().unwrap())
                    },
                    created_at: {
                        let s: String = row.get(4)?;
                        Some(s.parse::<DateTime<Utc>>().unwrap())
                    },
                    tags: HashMap::new(),
                })
            })
    }

    pub fn get_all_records_from_database(&self) -> rusqlite::Result<Vec<Record>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, modified_at, created_at FROM record"
        )?;

        let records_iter = stmt.query_map([], |row| {

            let id: i32 = row.get(0)?;

            let modified_at = row.get::<_, Option<String>>(2)?.and_then(|s| {
                if s.is_empty() { None } else { Some(s.parse::<DateTime<Utc>>().unwrap()) }
            });

            let created_at = row.get::<_, Option<String>>(3)?.and_then(|s| {
                if s.is_empty() { None } else { Some(s.parse::<DateTime<Utc>>().unwrap()) }
            });

            let tag_map = self.get_tags_for_record(id)?;

            Ok(Record {
                id,
                title: row.get(1)?,
                content: None,
                modified_at,
                created_at,
                tags: tag_map,
            })
        })?;

        let mut records = Vec::new();
        for rec in records_iter {
            records.push(rec?);
        }

        Ok(records)
    }

   
    pub fn remove_record(&self, id: i32) -> rusqlite::Result<()> {
        self.conn.execute(
            "DELETE FROM record WHERE id = ?1",
            [id],
        )?;
        Ok(())
    }

    pub fn insert_tag(&self, tag: &Tag) -> rusqlite::Result<i32> {
        self.conn.execute(
            "INSERT OR IGNORE INTO tag (tag) VALUES (?1)",
            [&tag.name],
        )?;

        let id: i32 = self.conn.query_row(
            "SELECT id FROM tag WHERE tag = ?1",
            [&tag.name],
            |row| row.get(0),
        )?;
        Ok(id)
    }

    pub fn add_tag_to_record(&self, record_id: i32, tag_name: &str) -> rusqlite::Result<()> {
        let tag_id = self.insert_tag(&Tag::new(tag_name))?;
        self.conn.execute(
            "INSERT OR IGNORE INTO relation (record_id, tag_id) VALUES (?1, ?2)",
            (record_id, tag_id),
        )?;
        Ok(())
    }

    pub fn remove_tag_from_record(&self, record_id: i32, tag_name: &str) -> rusqlite::Result<()> {
        let tag_id: Option<i32> = self.conn.query_row(
            "SELECT id FROM tag WHERE tag = ?1",
            [&tag_name],
            |row| row.get(0),
        ).optional()?; 

        if let Some(id) = tag_id {
            self.conn.execute(
                "DELETE FROM relation WHERE record_id = ?1 AND tag_id = ?2",
                (record_id, id),
            )?;
        }
        Ok(())
    }

    pub fn get_tags_for_record(&self, record_id: i32) -> rusqlite::Result<HashMap<String,bool>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.tag
             FROM tag t
             JOIN relation r ON t.id = r.tag_id
             WHERE r.record_id = ?1"
        )?;

        let mut tags = HashMap::new();
        for tag in stmt.query_map([record_id], |row| row.get::<_, String>(0))? {
            tags.insert(tag?, true);
        }
        Ok(tags)
    }

}


#[derive(Debug)]
pub struct Record {
    id: i32,
    title: Option<String>,
    content: Option<String>,
    modified_at: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    tags: HashMap<String, bool>,
}

impl Record {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        let now = Utc::now();
        Self { id: -1, title, content , modified_at: Some(now), created_at: Some(now), tags: HashMap::new() } // time to implement
    }

    pub fn empty() -> Self {
        Self { id: -1, title: None, content: None, modified_at: None, created_at: None, tags: HashMap::new() }
    }
}

#[derive(Debug)]
pub struct Tag {
    id: i32,
    name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self { id: -1, name: name.to_string() }
    }
}

use std::fmt;
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        let tags: String = self.tags.keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "ID: {}; TITLE: {}; UPDATED AT: {}; CREATED AT: {}; TAG: {}", self.id, self.title.clone().unwrap(), self.modified_at.unwrap(), self.created_at.unwrap(), tags)
    }
}
