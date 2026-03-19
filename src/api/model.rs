use chrono::{DateTime, Local, Utc};

pub struct Note {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    modified_at: Option<String>,
    created_at: Option<String>
}

impl Note {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        let now = Some(data_to_readable(Utc::now()));
        Self {
            id: None,
            title,
            content,
            modified_at: now.clone(),
            created_at: now
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


fn data_to_readable(data: DateTime<Utc>) -> String {
    data.with_timezone(&Local)
        .format("%H:%M %Y-%m-%d")
        .to_string()
}
