use clap::Parser;

pub mod app;
pub mod argument_parser;
pub mod database;
pub mod ui;

use argument_parser::{Args, Commands};
use database::{Database, Record};

use crate::ui::simpleui::{add_record, list_records, remove_record, show_record};

fn main() -> rusqlite::Result<()> {
    let args = argument_parser::Args::parse();

    let db = Database::new("rbrain.db");

    match args.command {
        Some(Commands::Ls) => list_records(&db)?,
        Some(Commands::Add { title: None }) => add_record(&db, None)?,
        Some(Commands::Add { title: Some(title) }) => add_record(&db, Some(title))?,
        Some(Commands::Rm { id }) => remove_record(&db, id)?,
        Some(Commands::Show { id }) => show_record(&db, id)?,
        Some(Commands::Mod { encrypt, decrypt }) => println!("Modify: {}", encrypt || decrypt),
        None => {}
    }

    Ok(())

    // if args.encrypt == true {
    //     println!("Encrypt");
    // } else if args.decrypt == true {
    //     println!("Decrypt");
    // }
}
