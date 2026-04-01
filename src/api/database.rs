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
            Some(content) => content,
            None => &"".to_string(),
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
            "INSERT INTO note (title, content, created_at, modified_at) VALUES (?1, ?2, ?3, ?4)",
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
                    content: row.get(2)?,
                    modified_at: Some(row.get(3)?),
                    created_at: Some(row.get(4)?)
                })
            },
        )

    }

    pub fn get_all_notes_from_database(&self) -> rusqlite::Result<Vec<Note>> {
        let mut stmt = self.conn
            .prepare("SELECT id, title, created_at, modified_at FROM note")?;
        let note_iter = stmt.query_map([], |row| {
            let id: Option<i32> = row.get(0)?; 
            let title = row.get(1)?;
            let created_at = row.get(2)?;
            let modified_at = row.get(3)?;

            Ok(Note {
                id,
                title,
                content: None,
                created_at,
                modified_at
            })
        })?;
        
        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note?);
        }

        Ok(notes)
    }
}


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn note_in_and_out() -> rusqlite::Result<()> {
        let file = "innouttest.db";

        let db = Database::new(file);
        let title = "Test title".to_string();
        let content = "Test content".to_string();

        let note = Note::new(Some(title.clone()), Some(content.clone()));

        db.insert_note_to_database(&note)?;

        let loaded_note = db.get_note_from_database(1)?;

        assert_eq!(
            loaded_note.title,
            Some(title)
        );
        assert_eq!(
            loaded_note.content,
            Some(content)
        );

        fs::remove_file(file).expect("Error with deleting test database");
        Ok(())
    }

    #[test]
    fn get_all_notes() ->rusqlite::Result<()> {
        let file = "getallnotestest.db";

        let db = Database::new(file);
        let title1 = Some("Test title 1".to_string());
        let title2 = Some("Test title 2".to_string());

        let note1 = Note::new(title1.clone(), None);
        let note2 = Note::new(title2.clone(), None);

        db.insert_note_to_database(&note1)?;
        db.insert_note_to_database(&note2)?;

        let notes = db.get_all_notes_from_database().unwrap();

        let t1 = notes.first().unwrap();
        let t2 = notes.get(1).unwrap();

        assert_eq!(
            title1,
            t1.title
        );

        assert_eq!(
            title2,
            t2.title
        );

        fs::remove_file(file).expect("Error with deleting test database");
        Ok(())

    }

}
