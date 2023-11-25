use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct MyArgs {
    #[clap(subcommand)]
    pub entry: EntryType,
}

#[derive(Debug, Subcommand)]
pub enum EntryType {
    Create(CreateCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub text: String,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    pub id: i32,
}
