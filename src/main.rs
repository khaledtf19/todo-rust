use anyhow::{Context, Result};
use clap::Parser;
extern crate dotenv;
use dotenv::dotenv;
use sqlx::Row;
use std::env;

mod args;

use args::{CreateCommand, DeleteCommand, EntryType,  MyArgs, DoneIdCommand,DoneNameCommand, DoneType, DoneCommand};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Connecting to the DB");
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let args = MyArgs::parse();
    match args.entry {
        EntryType::Create(CreateCommand { text }) => {}
        EntryType::Done(done) => {
            match done.done {
                DoneType::Id(DoneIdCommand {id}) => {
                    println!("id: {:?}", id);
                }
                DoneType::Name(DoneNameCommand{name})=> {
                    println!("name: {:?}", name);
                }
            }
            // println!("id: {:?}", s);
        }

    }

    Ok(())
}
