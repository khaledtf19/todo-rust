use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct MyArgs {
    #[clap(subcommand)]
    pub entry: EntryType,
}

#[derive(Debug, Subcommand)]
pub enum EntryType {
    Create(CreateCommand),
    Done(DoneCommand),
}

#[derive(Debug, Args)]
pub struct DoneCommand {
    #[clap(subcommand)]
    pub done: DoneType,
}

#[derive(Debug, Subcommand)]
pub enum DoneType {
    Id(DoneIdCommand),
    Name(DoneNameCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    pub name: String,
    pub discription: String,
}

#[derive(Debug, Args)]
pub struct DoneNameCommand {
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DoneIdCommand {
    pub id: i32,
}

#[derive(Debug, Args)]
pub struct DeleteCommand {
    pub id: i32,
}
