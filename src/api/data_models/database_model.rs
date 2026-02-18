use std::fmt;
use chrono::{Utc};

pub struct Record {
    pub id: i32,
    pub title: String,
    pub content: String, 
    pub tags: Vec<String>,
    pub relation: Vec<i32>,
    pub updated_at: String,
    pub created_at: String
}

impl Record {
    pub fn new(title: &str, content: &str) -> Self {
        let now = Utc::now().format("%H:%M %d-%m-%Y").to_string();
        Record {
            id: 0,
            title: title.into(),
            content: content.into(),
            tags: Vec::new(),
            relation: Vec::new(),
            updated_at: now.clone(),
            created_at: now,
        }
        
    }

    pub fn display(&self) {
        println!("ID: {} TITLE: \n{}\nCONTENT: {}\n------------------------\nTAGS: {}\nUPDATED AT {}\nCREATED AT {}",
            self.id,
            self.title,
            self.content,
            self.tags.join(", "),
            self.updated_at,
            self.created_at
        )
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        write!(f, "ID: {} | Title: {} | Tags: {} | Updated at: {} | Created at: {}", self.id, self.title, self.tags.join(","), self.updated_at, self.created_at)
    }
}


