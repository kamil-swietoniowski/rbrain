use clap::Parser;

pub mod app;
pub mod argument_parser;
pub mod database;
pub mod ui;
pub mod encrypt;
pub mod utility;

use argument_parser::{Args, Commands};
use database::{Database, Record};

use crate::{app::App, encrypt::encrypt, ui::simpleui::{add_record, list_records, remove_record, show_record}};

fn main() -> rusqlite::Result<()> {
    let app App::new()
    Ok(())
}
