use chrono::{DateTime, Local, Utc};

pub struct Note {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub modified_at: Option<String>,
    pub created_at: Option<String>,
    pub categories: Option<Vec<i32>>
}

pub struct Category {
    pub id: Option<i32>,
    pub name: String
}

impl Note {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        let now = Some(data_to_readable(Utc::now()));
        Self {
            id: None,
            title,
            content,
            modified_at: now.clone(),
            created_at: now,
            categories: Some(vec![])
        }
    }

    pub fn from_querry() {
        todo!()
    }

    pub fn display(&self, extra_info: bool) {
        let title = if let Some(t) = &self.title {
            t
        } else {
            &"Not Specified".to_string()
        };
        let content = if let Some(c) = &self.content {
            c
        } else {
            &"Not Specified".to_string()
        };

        if extra_info {
            println!("ID: {}; {}\n{}\nModified at: {}\nCreated at: {}", self.id.unwrap(), title, content, self.modified_at.clone().unwrap(), self.created_at.clone().unwrap())
        } else {
            println!("{}\n{}", title, content);
        }
    }
}

impl Category {
    pub fn new(name: String) -> Self {
        Self {id: None, name}
    }
}


fn data_to_readable(data: DateTime<Utc>) -> String {
    data.with_timezone(&Local)
        .format("%H:%M %Y-%m-%d")
        .to_string()
}
