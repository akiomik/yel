use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    /// List highlights and notes
    List {},

    /// Search highlights and notes
    Search { query: String },
}
