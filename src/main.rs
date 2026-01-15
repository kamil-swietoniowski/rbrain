use clap::Parser;

pub mod database;
pub mod argument_parser;
pub mod app;
pub mod ui;

use database::{Database, Record};
use argument_parser::{Args, Commands};

use crate::ui::simpleui::{add_record, list_records, remove_record, show_record};

fn main() -> rusqlite::Result<()> {
    let args = argument_parser::Args::parse();
    
    let db = Database::new("rbrain.db");
    
    match args.command {
        Some(Commands::Ls) => list_records(&db)?,
        Some(Commands::Add) => add_record(&db)?,
        Some(Commands::Rm) => remove_record(&db)?,
        Some(Commands::Show) => show_record(&db)?,
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
