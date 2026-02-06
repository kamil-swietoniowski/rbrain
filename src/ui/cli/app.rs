use crate::{
    api::{
        database::database::Database,
        model::model::{Record, Tag},
    },
    ui::cli::argument_parser::{Action, Append, Args, Encrypt, LookFor, Source, TagAct},
};
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

pub struct App {
    pub database: Database,
    pub current_record: Record,
    pub list_of_records: Vec<Record>,
    pub all_tags: Vec<Tag>,
}

// enum Action {
//     Operatian((Option<String>, Source)),
//     Show(Option<i32>),
//     Delete(Option<i32>),
//     List,
//     Menu,
// }
//
// enum Source {
//     Menu,
//     File(String),
//     Pipe,
//     Argument(String),
//     AddTC,
// }
//
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
        let status = Args::collect();
        match status {
            Action::Ls => list_records(self),
            Action::Get(id) => get_record(self, id),
            Action::Add(title, source, encryption) => add_record(self, title, source, encryption),
            Action::Lf(lookfor) => look_for(self, lookfor),
            Action::Rm(id, force) => remove_record(self, id, force),
            Action::Mod(id, title, content, append, enc) => {
                modification_record(self, id, title, content, append, enc)
            }
            Action::Tag(id, TagAct::Add(tag), _) => add_tag(self, id, tag),
            Action::Tag(id, TagAct::Remove(tag), force) => remove_tag(self, id, tag, force),
            Action::Tag(_, TagAct::List, _) => list_tags(self),
            _ => menu(self),
        }
    }
}

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
pub fn get_record(app: &App, id: Option<i32>) {
    let id = get_parse_id(id);
    let record = match app.database.get_record_from_database(id) {
        Ok(rec) => rec,
        Err(err) => {
            eprintln!("Couldn't find if or other problem: {}", err);
            return;
        }
    };
    record.display();
}
pub fn add_record(app: &App, title: Option<String>, source: Source, encrypt: bool) {
    let title = match title {
        Some(title) => Some(title),
        None => Some(input("Enter a title: ")),
    };

    let content = match source {
        Source::Content(text) => text,
        Source::File(Some(file)) => {
            let file = Path::new(&file);
            if !file.exists() {
                eprintln!("File do not exist");
            };
            Some(fs::read_to_string(file).unwrap())
        }
        Source::NotSpecified => Some(input("Enter text: ")),
        _ => {
            eprintln!("Problem occured");
            std::process::exit(1);
        }
    };

    let record = Record::new(title, content);
    app.database
        .insert_record_to_database(&record)
        .expect("Error while accessing Database");
}
pub fn remove_record(app: &App, id: Option<i32>, force: bool) {
    let id = get_parse_id(id);
    if !force && input("Are you sure you want to delete record(y/n): ").to_lowercase() != "y" {
        println!("Cancelled");
        return;
    }

    let _ = app.database.remove_record(id);
}
pub fn look_for(app: &App, look_for: LookFor) {}
pub fn modification_record(
    app: &App,
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    append: Append,
    encrypt: Encrypt,
) {
    let id = get_parse_id(id);

    let title = match title {
        Some(title) => Some(title),
        None => Some(input("Enter title: ")),
    };

    let content = if append != Append::NotSpecified {
        let record = app
            .database
            .get_record_from_database(id)
            .expect("Record does not exist");
        let mut content = record.content.unwrap_or("".to_string());
        match append {
            Append::Append(text) => content.push_str(&text),
            Append::AppendFile(file) => {
                let path = Path::new(&file);
                let text = fs::read_to_string(path).expect("Error accessing the file");
                content.push_str(&text);
            }
            _ => {}
        }
        content
    } else {
        match content {
            Some(content) => content,
            None => input("Enter content: "),
        }
    };

    let content = Some(content);
    let _ = app.database.modify_record(id, title, content);
}

fn add_tag(app: &App, id: Option<i32>, tag: Option<String>) {
    let id = get_parse_id(id);
    let tag = match tag {
        Some(tag) => tag,
        None => input("Enter TAG name: "),
    };

    match app.database.add_tag_to_record(id, &tag) {
        Ok(_) => eprintln!("Successful"),
        Err(e) => eprintln!("Something went wrong: {}", e),
    };
}
fn remove_tag(app: &App, id: Option<i32>, tag: Option<String>, force: bool) {
    let id = get_parse_id(id);
    let tag = match tag {
        Some(tag) => tag,
        None => input("Enter TAG name: "),
    };

    if !force && input("Are you sure you want to delete tag(y/n): ") != "y" {
        println!("Cancelled");
        return;
    }

    match app.database.remove_tag_from_record(id, &tag) {
        Ok(_) => eprintln!("Successful"),
        Err(e) => eprintln!("Something went wrong: {}", e),
    };
}
fn list_tags(app: &App) {
    let tags = app.database.get_all_tags().unwrap_or_else(|err| {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    });

    if tags.is_empty() {
        eprintln!("Empty list");
        return;
    }

    for tag in tags {
        println!("{tag}");
    }
}

pub fn menu(app: &mut App) {
    loop {
        println!("\nRbrain\n1. List Records\n2. Show Record\n3. Add Record\n4. Delete Record\n5. Add Tag to Record\n6. Delete Tag from Record\n7. List all Tags\n8. Exit");
        let user_option = input("I choose: ");

        match user_option.as_str() {
            "1" => list_records(app),
            "2" => get_record(app, None),
            "3" => add_record(app, None, Source::NotSpecified, false),
            "4" => remove_record(app, None, false),
            "5" => add_tag(app, None, None),
            "6" => remove_tag(app, None, None, false),
            "7" => list_tags(app),
            "8" => break,
            _ => {
                eprintln!("Wrong option");
                continue;
            }
        }
    }
}

fn input(querry: &str) -> String {
    let mut buf = String::new();
    print!("{querry}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buf).expect("Error with input");
    buf.trim().to_string()
}

fn get_parse_id(id: Option<i32>) -> i32 {
    loop {
        match id {
            Some(id) => return id,
            None => {
                return match input("Enter ID: ").parse::<i32>() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("Not correct ID: ");
                        continue;
                    }
                }
            }
        }
    }
}
