use anyhow::Result;
use clap::Parser;
use sea_orm::Database;
use tabled::Table;

use yel::{BookCommand, BookRepository, Cli, Command, HighlightCommand};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Book(args)) => match &args.command {
            Some(BookCommand::List {}) => {
                let db = Database::connect("sqlite://./db/BKLibrary.sqlite?mode=ro").await?;
                let repo = BookRepository::new(db);
                let books = repo.find_all().await?;
                let table = Table::new(books);

                println!("{table}");
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

    Ok(())
}
