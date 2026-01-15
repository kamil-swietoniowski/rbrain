use clap::Parser;

pub mod database;
pub mod argument_parser;
pub mod app;

use database::{Database, Record};
use argument_parser::{Args, Commands};

fn main() {
    let args = argument_parser::Args::parse();

    match args.command {
        Some(Commands::Ls) => println!("List"),
        Some(Commands::Add) => println!("Add"),
        Some(Commands::Rm) => println!("Remove"),
        Some(Commands::Show) => println!("Show"),
        Some(Commands::Mod { encrypt, decrypt }) => println!("Modify: {}", encrypt || decrypt),
        None => {}
        _ => {}
    }
    let db = Database::new("rbrain.db");

    // let mut title = String::new();
    // println!("Podaj tytuł: ");
    // std::io::stdin().read_line(&mut title).unwrap();
    //
    // let mut content = String::new();
    // println!("Podaj content: ");
    // std::io::stdin().read_line(&mut content).unwrap();
    //
    // let osoba1 = Record::new(Some(title.trim().to_string()), Some(content.trim().to_string()));
    // let _ = db.insert_record_to_database(&osoba1);

    db.add_tag_to_record(5, "Test");
    db.add_tag_to_record(6, "Test");
    let people = db.get_all_records_from_database().unwrap();

    println!("Found {} records", people.len());
    for n in people {
        println!("{}", n);
    }

    // if args.encrypt == true {
    //     println!("Encrypt");
    // } else if args.decrypt == true {
    //     println!("Decrypt");
    // }
}
