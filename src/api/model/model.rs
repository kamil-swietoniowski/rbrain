use chrono::{DateTime, Local, Utc};
use std::collections::HashMap;

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
    pub id: i32,
    pub name: String,
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
