use chrono::{offset::Utc, DateTime, Local};
use rusqlite::{Connection, OptionalExtension, Result};
use std::collections::HashMap;

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
    pub fn new(path: &str) -> Self {
        let conn = init_db(path).unwrap_or_else(|error| {
            eprintln!("Error occured: {}", error);
            std::process::exit(0);
        });
        Self { conn }
    }
    pub fn insert_record_to_database(&self, record: &Record) -> rusqlite::Result<()> {
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

    pub fn modify_record(
        &self,
        id: i32,
        title: Option<String>,
        content: Option<String>,
    ) -> rusqlite::Result<()> {
        let modified_at = Utc::now().to_rfc3339();

        if title.is_some() && content.is_some() {
            self.conn.execute(
                "UPDATE record SET title = ?1, content = ?2, modified_at = ?3 WHERE id = ?4",
                (title.unwrap(), content.unwrap(), modified_at, id),
            )?;
        } else if title.is_some() {
            self.conn.execute(
                "UPDATE record SET title = ?1, modified_at = ?2 WHERE id = ?3",
                (title.unwrap(), modified_at, id),
            )?;
        } else if content.is_some() {
            self.conn.execute(
                "UPDATE record SET content = ?1, modified_at = ?2 WHERE id = ?3",
                (content.unwrap(), modified_at, id),
            )?;
        }
        Ok(())
    }

    pub fn get_record_from_database(&self, id: i32) -> rusqlite::Result<Record> {
        self.conn.query_row(
            "SELECT id, title, content, modified_at, created_at FROM record WHERE id = ?1",
            [id],
            |row| {
                Ok(Record {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: {
                        let content: Option<String> = row
                            .get::<_, Option<Vec<u8>>>(2)?
                            .and_then(|bytes| String::from_utf8(bytes).ok());
                        content
                    },
                    modified_at: {
                        let s: String = row.get(3)?;
                        Some(s.parse::<DateTime<Utc>>().unwrap())
                    },
                    created_at: {
                        let s: String = row.get(4)?;
                        Some(s.parse::<DateTime<Utc>>().unwrap())
                    },
                    tags: self.get_tags_for_record(row.get(0)?)?,
                })
            },
        )
    }

    pub fn get_all_records_from_database(&self) -> rusqlite::Result<Vec<Record>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, modified_at, created_at FROM record")?;

        let records_iter = stmt.query_map([], |row| {
            let id: i32 = row.get(0)?;

            let modified_at = row.get::<_, Option<String>>(2)?.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<DateTime<Utc>>().unwrap())
                }
            });

            let created_at = row.get::<_, Option<String>>(3)?.and_then(|s| {
                if s.is_empty() {
                    None
                } else {
                    Some(s.parse::<DateTime<Utc>>().unwrap())
                }
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
        self.conn
            .execute("DELETE FROM record WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn insert_tag(&self, tag: &Tag) -> rusqlite::Result<i32> {
        self.conn
            .execute("INSERT OR IGNORE INTO tag (tag) VALUES (?1)", [&tag.name])?;

        let id: i32 =
            self.conn
                .query_row("SELECT id FROM tag WHERE tag = ?1", [&tag.name], |row| {
                    row.get(0)
                })?;
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
        let tag_id: Option<i32> = self
            .conn
            .query_row("SELECT id FROM tag WHERE tag = ?1", [&tag_name], |row| {
                row.get(0)
            })
            .optional()?;

        if let Some(id) = tag_id {
            self.conn.execute(
                "DELETE FROM relation WHERE record_id = ?1 AND tag_id = ?2",
                (record_id, id),
            )?;
        }
        Ok(())
    }

    pub fn get_tags_for_record(&self, record_id: i32) -> rusqlite::Result<HashMap<String, bool>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.tag
             FROM tag t
             JOIN relation r ON t.id = r.tag_id
             WHERE r.record_id = ?1",
        )?;

        let mut tags = HashMap::new();
        for tag in stmt.query_map([record_id], |row| row.get::<_, String>(0))? {
            tags.insert(tag?, true);
        }
        Ok(tags)
    }

    pub fn remove_tag_from_database(&self, tag: &str) -> rusqlite::Result<()> {
        self.conn.execute("DELETE FROM tag WHERE tag = ?1", [tag])?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Record {
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub modified_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub tags: HashMap<String, bool>,
}

impl Record {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: -1,
            title,
            content,
            modified_at: Some(now),
            created_at: Some(now),
            tags: HashMap::new(),
        }
    }

    pub fn empty() -> Self {
        Self {
            id: -1,
            title: None,
            content: None,
            modified_at: None,
            created_at: None,
            tags: HashMap::new(),
        }
    }

    pub fn display(&self) {
        println!("ID: {}", self.id);
        println!(
            "Tags: {}",
            self.tags
                .keys()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("Title:\n{}", self.title.clone().unwrap());
        println!("\nContent:\n{}", self.content.clone().unwrap());
        println!(
            "\nUpdated At: {}\nCreated At: {}",
            data_to_readable(self.modified_at.unwrap()),
            data_to_readable(self.created_at.unwrap())
        );
    }
}

#[derive(Debug)]
pub struct Tag {
    id: i32,
    name: String,
}

impl Tag {
    pub fn new(name: &str) -> Self {
        Self {
            id: -1,
            name: name.to_string(),
        }
    }
}

use std::fmt;
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tags: String = self
            .tags
            .keys()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "ID: {}; TITLE: {}; UPDATED AT: {}; CREATED AT: {}; TAG: {}",
            self.id,
            self.title.clone().unwrap(),
            data_to_readable(self.modified_at.unwrap()),
            data_to_readable(self.created_at.unwrap()),
            tags
        )
    }
}

fn data_to_readable(data: DateTime<Utc>) -> String {
    data.with_timezone(&Local)
        .format("%H:%M %Y-%m-%d")
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs, io::ErrorKind};

    use super::*;

    #[test]
    fn record_in_and_out_test() -> rusqlite::Result<()> {
        let db = Database::new("test.db");
        let title = "This is test record title".to_string();
        let content = "This is test record content".to_string();
        let testrecord = Record::new(Some(title), Some(content));

        db.insert_record_to_database(&testrecord)?;
        let loaded_record = db.get_record_from_database(1)?;

        assert_eq!(
            loaded_record.title,
            Some("This is test record title".to_string())
        );
        assert_eq!(
            loaded_record.content,
            Some("This is test record content".to_string())
        );
        fs::remove_file("test.db").expect("File with deleting test database");
        Ok(())
    }

    #[test]
    fn database_list_of_records_test() -> rusqlite::Result<()> {
        let db = Database::new("test2.db");

        let records: Vec<Record> = vec![
            Record::new(Some("This is test record title".to_string()), None),
            Record::new(Some("This is test record title2".to_string()), None),
            Record::new(Some("This is test record title3".to_string()), None),
            Record::new(Some("This is test record title4".to_string()), None),
            Record::new(Some("This is test record title5".to_string()), None),
        ];

        for record in &records {
            db.insert_record_to_database(record)?;
        }

        let loaded_records = db.get_all_records_from_database()?;

        for (i, record) in loaded_records.iter().enumerate() {
            assert_eq!(records[i].title, record.title)
        }

        fs::remove_file("test2.db").expect("File with deleting test database");

        Ok(())
    }

    #[test]
    fn remove_record_test() -> rusqlite::Result<()> {
        fs::remove_file("test3.db").ok();
        let db = Database::new("test3.db");

        let testrecord = Record::new(Some("This is test record title".to_string()), None);

        db.insert_record_to_database(&testrecord)?;
        let loaded_records = db.get_all_records_from_database()?;

        assert_eq!(loaded_records.len(), 1);

        db.remove_record(1)?;
        let loaded_records2 = db.get_all_records_from_database()?;

        assert_eq!(loaded_records2.len(), 0);

        fs::remove_file("test3.db").expect("File with deleting test database");

        Ok(())
    }

    #[test]
    fn tag_into_record_test() -> rusqlite::Result<()> {
        let db = Database::new("test4.db");

        let testrecord = Record::new(Some("This is test record title".to_string()), None);

        db.insert_record_to_database(&testrecord)?;

        db.add_tag_to_record(1, "Test Tag")?;

        let record = db.get_record_from_database(1)?;

        fs::remove_file("test4.db").expect("Problem deleting test database");

        let mut testhashmap = HashMap::new();
        testhashmap.insert("Test Tag".to_string(), true);

        assert_eq!(record.tags, testhashmap);

        Ok(())
    }

    #[test]
    fn remove_tag_from_record_test() -> rusqlite::Result<()> {
        let db = Database::new("test5.db");

        let testrecord = Record::new(Some("This is test record".to_string()), None);
        db.insert_record_to_database(&testrecord)?;

        let mut tags = HashMap::new();
        tags.insert("Test Tag".to_string(), true);

        db.add_tag_to_record(1, "Test Tag")?;
        let tag = db.get_record_from_database(1)?.tags;

        assert_eq!(tag, tags);

        db.remove_tag_from_record(1, "Test Tag")?;
        let tag = db.get_record_from_database(1)?.tags;
        fs::remove_file("test5.db").expect("Problem deleting test database");
        assert_ne!(tag, tags);

        Ok(())
    }

    #[test]
    fn remove_tag() -> rusqlite::Result<()> {
        let db = Database::new("test6.db");

        let testrecord = Record::new(Some("This is test record".to_string()), None);
        db.insert_record_to_database(&testrecord)?;

        let mut tags = HashMap::new();
        tags.insert("Test Tag".to_string(), true);

        db.add_tag_to_record(1, "Test Tag")?;
        let tag = db.get_record_from_database(1)?.tags;

        assert_eq!(tag, tags);

        db.remove_tag_from_database("Test Tag")?;
        let tag = db.get_record_from_database(1)?.tags;
        fs::remove_file("test6.db").expect("Problem deleting test database");
        assert_ne!(tag, tags);

        Ok(())
    }
}
