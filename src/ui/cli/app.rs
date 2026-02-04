use chrono::format::InternalNumeric;
use clap::Parser;

use crate::{
    api::{
        database::database::Database,
        model::model::{Record, Tag},
    },
    ui::cli::argument_parser::{Args, Commands},
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

enum Action {
    Operatian((Option<String>, Source)),
    Show(Option<i32>),
    Delete(Option<i32>),
    List,
    Menu,
}

enum Source {
    Menu,
    File(String),
    Pipe,
    Argument(String),
    AddTC,
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
        let status = Self::define_content_source();
        match status {
            Action::List => list_all_records(self),
            Action::Show(id) => show_record(self, id),
            Action::Delete(id) => delete_record(self, id),
            Action::Menu => menu(self),
            Action::Operatian((title, source)) => match (title, source) {
                (_, Source::Menu) => menu(self),
                (None, Source::File(file)) => add_record_by_file(self, None, file),
                (title, Source::File(file)) => add_record_by_file(self, title, file),
                (Some(title), Source::Argument(content)) => {
                    add_record_by_argument(self, Some(title), content)
                }
                (None, Source::Argument(content)) => add_record_by_argument(self, None, content),
                (Some(title), Source::Pipe) => add_record_by_pipe(self, title),
                (None, Source::AddTC) => add_c_t_fn(self),
                _ => {}
            },
        }
    }
    pub fn define_content_source() -> Action {
        // To implement Pipe
        let args = Args::parse();

        match args.command {
            Commands::List => Action::List,
            Commands::Show { id } => Action::Show(id),
            Commands::Delete { id } => Action::Delete(id),
            Commands::Add {
                title,
                content,
                file,
            } => {
                if file.is_some() {
                    return Action::Operatian((title, Source::File(file.unwrap())));
                } else if content.is_some() {
                    return Action::Operatian((title, Source::Argument(content.unwrap())));
                } else {
                    return Action::Operatian((None, Source::AddTC));
                }
            }
            Commands::Menu => Action::Menu,
        }
        // if args.list {
        //     return Action::List;
        // } else if args.show.is_some() {
        //     return Action::Show(args.show.unwrap());
        // }
        //
        // let title = args.title;
        //
        // if args.delete.is_some() {
        //     return Action::Delete(args.delete.unwrap());
        // }
        //
        // if args.content.is_some() {
        //     return Action::Operatian((title, Source::Argument(args.content.unwrap())));
        // } else if args.file.is_some() {
        //     let filestring = args.file.unwrap();
        //     let file = Path::new(&filestring);
        //     if file.is_file() {
        //         return Action::Operatian((title, Source::File(filestring)));
        //     }
        // }
        // Action::Operatian((title, Source::Menu))
    }
}

fn add_c_t_fn(app: &App) {
    let title = Some(input("Enter title: "));
    let content = Some(input("Enter content: "));

    let record = Record::new(title, content);
    app.database.insert_record_to_database(&record).unwrap();
}

fn get_id(id: Option<i32>) -> i32 {
    match id {
        Some(id) => id,
        None => input("Enter ID: ").parse::<i32>().expect("Enter an number"),
    }
}

fn list_all_records(app: &App) {
    let list_of_records = app.database.get_all_records_from_database().unwrap();
    if list_of_records.is_empty() {
        eprintln!("Empty list");
        return;
    }

    for record in &list_of_records {
        println!("{record}")
    }
}
fn show_record(app: &App, id: Option<i32>) {
    let id = get_id(id);

    let record = match app.database.get_record_from_database(id) {
        Ok(rec) => rec,
        Err(err) => {
            eprintln!("Couldn't find if or other problem: {}", err);
            return;
        }
    };
    record.display();
}
fn delete_record(app: &App, id: Option<i32>) {
    let id = get_id(id);
    let choice = input("Are you sure you want to delete?: (y/n)");
    if choice.trim() != "y" {
        return;
    }
    let _ = app.database.remove_record(id);
}
fn add_record_by_file(app: &App, title: Option<String>, file: String) {
    let title = match title {
        Some(title) => Some(title),
        None => Some(input("Enter a title: ")),
    };

    let file = Path::new(&file);

    let content = Some(fs::read_to_string(file).expect("Can't access file"));

    let record = Record::new(title, content);
    app.database.insert_record_to_database(&record).unwrap();
}
fn add_record_by_argument(app: &App, title: Option<String>, text: String) {
    let title = match title {
        Some(title) => Some(title),
        None => Some(input("Enter a title: ")),
    };

    let record = Record::new(title, Some(text));
    app.database.insert_record_to_database(&record).unwrap();
}
fn add_record_by_pipe(app: &App, title: String) {}

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
