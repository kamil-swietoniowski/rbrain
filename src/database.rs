use std::collections::HashMap;

pub struct Record {
    id: i32,
    title: Option<String>,
    content: Option<String>,
    modified_at: Option<String>,
    create_at: Option<String>,
    tags: HashMap<String, bool>,
}

impl Record {
    pub fn load_titles_database() -> Self {
        todo!()
    }
    pub fn load_from_database() -> Self {
        todo!()
    }
}

