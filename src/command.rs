use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Manage highlights and notes
    #[command(arg_required_else_help = true)]
    Highlight(HighlightArgs),
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
