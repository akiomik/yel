use clap::Parser;

use yel::{BookCommand, Cli, Command, HighlightCommand};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Book(args)) => match &args.command {
            Some(BookCommand::List {}) => {
                println!("List");
            }
            Some(BookCommand::Search { query }) => {
                println!("Search with {query}");
            }
            None => {}
        },
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
