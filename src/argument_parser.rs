use clap::{Parser, Subcommand, ArgGroup};

#[derive(Subcommand)]
pub enum Commands {
    Ls,
    Add,
    Rm,
    Show,
    Mod {
        #[arg(long)]
        encrypt: bool,
        #[arg(long)]
        decrypt: bool,
    }
}

#[derive(Parser)]
#[command(name = "rbrain", version, about = "Local encrypted CLI notes in Rust", long_about = None,
    group(
        ArgGroup::new("case")
                .required(false) 
                .args(["encrypt", "decrypt"]),
    )
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(long = "encrypt")]
    pub encrypt: bool,

    #[arg(long = "decrypt")]
    pub decrypt: bool,
    // #[arg(short, long)]
    // name: String,
    //
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
}

