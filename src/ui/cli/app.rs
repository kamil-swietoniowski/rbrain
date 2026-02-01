use crate::api::{
    database::database::Database,
    model::model::{Record, Tag},
};

pub struct App {
    pub database: Database,
    pub current_record: Record,
    pub list_of_records: Vec<Record>,
    pub all_tags: Vec<Tag>,
}

impl App {
    pub fn new(database_name: &str) -> Self {
        let database = Database::new(database_name);
        let current_record = Record::empty();

        let list_of_records = database
            .get_all_records_from_database()
            .unwrap_or_else(|err| {
                eprintln!("Error accessing Database: {}", err);
                std::process::exit(1);
            });
        let all_tags: Vec<Tag> = Vec::new();

        Self {
            database,
            current_record,
            list_of_records,
            all_tags,
        }
    }
}
