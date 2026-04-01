use rusqlite::{Connection};
use crate::api::model::Note;


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

    pub fn insert_note_to_database(&self, note: &Note) -> rusqlite::Result<()> {
        let title = match &note.title {
            Some(title) => title,
            None => &"".to_string(),
        };
        let content = match &note.content {
            Some(content) => content.as_bytes().to_vec(),
            None => Vec::new(),
        };
        let modified_at = match &note.modified_at {
            Some(modified) => modified,
            None => &"".to_string(),
        };
        let created_at = match &note.created_at {
            Some(created) => created,
            None => &"".to_string(),
        };

        self.conn.execute(
            "INSERT INTO note (title, content, created_at, modified_at) FROM note VALUES (?1, ?2, ?3, ?4)",
            (title, content, modified_at, created_at)
        )?;
        Ok(())
    }
    
    pub fn get_note_from_database(&self, id: i32) -> rusqlite:: Result<Note> {
        self.conn.query_row(
            "SELECT id, title, content, modified_at, created_at FROM note WHERE id = ?1",
            [id],
            |row| {
                Ok(Note {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    content: {
                        let content: Option<String> = row
                            .get::<_, Option<Vec<u8>>>(2)?
                            .and_then(|bytes| String::from_utf8(bytes).ok());
                        content
                    },
                    modified_at: {
                        Some(row.get(3)?)
                    },
                    created_at: {
                        Some(row.get(4)?)
                    },
                })
            },
        )

    }




}
