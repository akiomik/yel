use clap::Parser;

use yel::{Cli, Command};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::List {}) => {
            println!("List");
        }
        Some(Command::Search {}) => {
            println!("Search");
        }
        None => {}
    }
}
