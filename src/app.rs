use crate::database::{Record, Database};
use crate::argument_parser::Args;

pub struct App {
    arguments: Args,
    database: Database,
    current_record: Option<Record>,
    loaded_records: Option<Vec<Record>>
}
