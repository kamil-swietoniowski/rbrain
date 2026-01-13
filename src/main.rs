use clap::Parser;

pub mod database;
pub mod argument_parser;
pub mod app;

use argument_parser::{Args, Commands};

fn main() {
    let args = argument_parser::Args::parse();

    match args.command {
        Commands::Ls => println!("List"),
        Commands::Add => println!("Add"),
        Commands::Rm => println!("Remove"),
        Commands::Show => println!("Show"),
        Commands::Mod { encrypt, decrypt } => println!("Modify: {}", encrypt || decrypt)
    }

    // if args.encrypt == true {
    //     println!("Encrypt");
    // } else if args.decrypt == true {
    //     println!("Decrypt");
    // }
}
