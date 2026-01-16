use crate::database::{Record, Tag};


pub fn search_record_by_tag(records: Vec<Record>, tag: &str) -> Option<Vec<Record>> {
    let mut result: Vec<Record> = Vec::new();

    for record in &records {
        if record.tags.contains_key(tag) {
            result.push(Record { id: record.id, title: record.title.clone(), content: record.content.clone(), modified_at: record.modified_at, created_at: record.created_at, tags: record.tags.clone() });
        }
    }

    if result.len() == 0 {
        return None;
    } 
    Some(result)
}

pub fn search_record_by_title(records: Vec<Record>, querry: &str) -> Option<Vec<Record>> {
    let mut result: Vec<Record> = Vec::new();

    for record in &records {
        let title = record.title.clone().unwrap().to_lowercase();
        if title.contains(querry) {
            result.push(Record { id: record.id, title: record.title.clone(), content: record.content.clone(), modified_at: record.modified_at, created_at: record.created_at, tags: record.tags.clone() });
        }
    }

    if result.len() == 0 {
        return None;
    } 
    Some(result)
}
