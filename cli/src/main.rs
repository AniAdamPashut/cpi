use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Sync {
        flags: String,
        name: String,
    }
}

fn main() {
    println!("Hello, world!");
}
