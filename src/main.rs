use clap::Parser;

use yel::{Cli, Command, HighlightCommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Highlight(args)) => match &args.command {
            Some(HighlightCommand::List {}) => {
                println!("List");
            }
            Some(HighlightCommand::Search { query }) => {
                println!("Search with {query}");
            }
            None => {}
        },
        None => {}
    }
}
