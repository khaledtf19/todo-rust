use anyhow::Result;
use clap::Parser;
extern crate dotenv;
use dotenv::dotenv;

mod args;
mod crud_todo;
mod structs;

use args::{CreateCommand, DeleteCommand, EntryType,  MyArgs};
use crud_todo::{create_todo, done_todo, delete_todo_by_id, get_all_todos};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Connecting to the DB");
    let url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let args = MyArgs::parse();

    match args.entry {
        EntryType::Create(CreateCommand { name,discription }) => {
            create_todo(name, discription, &pool).await?;
        }
        EntryType::Done(done) => {
            done_todo(done, &pool).await?;
        }
        EntryType::Delete(DeleteCommand { id }) => {
            delete_todo_by_id(id, &pool).await?;
        }
        EntryType::Get => {
            get_all_todos(&pool).await?;
        }
    }

    Ok(())
}
