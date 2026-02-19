use std::fmt;

use crate::api::utils::util_tools;

pub struct Record {
    pub id: i32,
    pub title: String,
    pub content: Option<String>, 
    pub tags: Vec<String>,
    pub relation: Vec<i32>,
    pub updated_at: String,
    pub created_at: String
}

impl Record {
    pub fn new(title: &str, content: &str) -> Self {
        let now = util_tools::now();
        Record {
            id: 0,
            title: title.into(),
            content: Some(content.into()),
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
            self.content.clone().unwrap_or("".into()),
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


