use clap::{ArgGroup, Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    Show {
        #[arg(short, long)]
        id: Option<i32>,
    },
    Delete {
        #[arg(short, long)]
        id: Option<i32>,
    },
    List,
    Add {
        #[arg(short, long)]
        title: Option<String>,

        #[arg(short, long, conflicts_with = "file")]
        content: Option<String>,

        #[arg(short, long, conflicts_with = "content")]
        file: Option<String>,
    },
    Menu,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
// , group(
//         ArgGroup::new("case").required(false).args(["content", "file"])
// ))]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
    // #[arg(short, long)]
    // pub list: bool,
    //
    // #[arg(short, long)]
    // pub show: Option<i32>,
    //
    // #[arg(short, long)]
    // pub delete: Option<i32>,
    //#[arg(short, long, default_value_t = 1)]
    //count: u8,
}
