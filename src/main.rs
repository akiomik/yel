use anyhow::Result;
use clap::Parser;
use crossterm::terminal;
use pager::Pager;
use sea_orm::Database;
use tabled::{
    settings::{peaker::PriorityMax, Settings, Style, Width},
    Table,
};

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
                let terminal_width = terminal::size()?.0 as usize;
                let table_settings = Settings::default()
                    .with(Width::wrap(terminal_width).priority(PriorityMax::default()))
                    .with(Width::increase(terminal_width));
                let mut table = Table::new(books);
                table.with(Style::modern()).with(table_settings);

                Pager::new().setup();
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
