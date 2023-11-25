use anyhow::{Context, Result};
use clap::Parser;
extern crate dotenv;
use dotenv::dotenv;
use sqlx::Row;
use std::env;

mod args;

use args::{CreateCommand, DeleteCommand, EntryType, MyArgs};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Connecting to the DB");
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    let args = MyArgs::parse();
    match args.entry {
        EntryType::Create(CreateCommand { text }) => {}
    }

    Ok(())
}
