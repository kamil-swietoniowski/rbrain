use crate::api::{
    database::database::Database,
    model::model::{Record, Tag},
};
use std::io::{self, Write};

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
    pub fn run(&mut self) {
        menu(self);
    }
}

pub fn menu(app: &mut App) {
    loop {
        println!("\nRbrain\n1. List Records\n2. Show Record\n3. Add Record\n4. Delete Record\n5. Add Tag to Record\n6. Delete Tag from Record\n7. List all Tags\n8. Exit");
        let user_option = input("I choose: ");

        match user_option.as_str() {
            "1" => menu::list_records(app),
            "2" => menu::show_record(app),
            "3" => menu::add_record(app),
            "4" => menu::del_record(app),
            "5" => menu::add_tag(app),
            "6" => menu::del_tag(app),
            "7" => todo!(),
            "8" => break,
            _ => {
                eprintln!("Wrong option");
                continue;
            }
        }
    }
}

mod menu {
    use crate::{
        api::model::model::{Record, Tag},
        ui::cli::app::{input, App},
    };
    pub fn list_records(app: &App) {
        let list_of_records = app.database.get_all_records_from_database().unwrap();

        if list_of_records.is_empty() {
            eprintln!("Empty list");
            return;
        }

        for record in &list_of_records {
            println!("{record}")
        }
    }
    pub fn show_record(app: &App) {
        let id = input("Enter Record ID: ");
        let id: i32 = match id.parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Not an correct ID: {}", err);
                return;
            }
        };
        let record = match app.database.get_record_from_database(id) {
            Ok(rec) => rec,
            Err(err) => {
                eprintln!("Couldn't find if or other problem: {}", err);
                return;
            }
        };
        record.display();
    }

    pub fn add_record(app: &App) {
        let title = Some(input("Enter title: "));
        let content = Some(input("Enter content: "));
        let record = Record::new(title, content);
        app.database.insert_record_to_database(&record).unwrap();
    }

    pub fn del_record(app: &App) {
        let id = input("Enter Record ID: ");
        let id: i32 = match id.parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Not an correct ID: {}", err);
                return;
            }
        };
        let _ = app.database.remove_record(id);
    }

    pub fn add_tag(app: &App) {
        let record_id = input("Enter Record ID: ");
        let tag_name = input("Enter TAG name: ");
        let record_id: i32 = match record_id.parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Not an correct ID: {}", err);
                return;
            }
        };

        app.database
            .add_tag_to_record(record_id, &tag_name)
            .unwrap();
    }

    pub fn del_tag(app: &App) {
        let record_id = input("Enter Record ID: ");
        let tag_name = input("Enter TAG name: ");
        let record_id: i32 = match record_id.parse() {
            Ok(num) => num,
            Err(err) => {
                eprintln!("Not an correct ID: {}", err);
                return;
            }
        };

        app.database
            .remove_tag_from_record(record_id, &tag_name)
            .unwrap();
    }

    //pub fn get_all_tags(app: &App) {
    //    let tags: Vec<Tag> = app.database.get_all_records_from_database().unwrap();
    //    for tag in tags {
    //        println!("{}", tag.name)
    //    }
    //}
}

fn input(querry: &str) -> String {
    let mut buf = String::new();
    print!("{querry}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buf).expect("Error with input");
    buf.trim().to_string()
}
