use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage books
    #[command(arg_required_else_help = true)]
    Book(BookArgs),

    /// Manage highlights and notes
    #[command(arg_required_else_help = true)]
    Highlight(HighlightArgs),
}

#[derive(Debug, Args)]
pub struct BookArgs {
    #[command(subcommand)]
    pub command: Option<BookCommand>,
}

#[derive(Debug, Subcommand)]
pub enum BookCommand {
    /// List books
    List {},

    /// Search books
    Search { query: String },
}

#[derive(Debug, Args)]
pub struct HighlightArgs {
    #[command(subcommand)]
    pub command: Option<HighlightCommand>,
}

#[derive(Debug, Subcommand)]
pub enum HighlightCommand {
    /// List highlights and notes
    List {},

    /// Search highlights and notes
    Search { query: String },
}
