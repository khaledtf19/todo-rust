use anyhow::Ok;
use anyhow::Result;
use sqlx::PgPool;
use sqlx::Row;

use crate::args::{DoneCommand, DoneIdCommand, DoneNameCommand, DoneType};
use crate::structs::Todo;

pub async fn create_todo(name: String, discription: String, pool: &PgPool) -> Result<()> {
    sqlx::query("INSERT INTO todos (name, discription) VALUES ($1, $2)")
        .bind(&name)
        .bind(&discription)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn done_todo(done: DoneCommand, pool: &PgPool) -> Result<()> {
    match done.done {
        DoneType::Id(DoneIdCommand { id }) => {
            let q = "SELECT * FROM todos WHERE id = $1;";
            let query = sqlx::query_as::<_, Todo>(q).bind(id);
            let todo = query.fetch_optional(pool).await?;
            println!("{:?}", todo);
        }
        DoneType::Name(DoneNameCommand { name }) => {
            let q = "SELECT * FROM todos WHERE name LIKE $1";
            let query = sqlx::query_as::<_, Todo>(q).bind(name + "%");
            let todos = query.fetch_all(pool).await?;
            println!("{:?}", todos);
        }
    }
    Ok(())
}

pub async fn search_todo_by_id(id: i32, pool: &PgPool) -> Result<()> {
    let q = "SELECT * FROM todos WHERE id = $1;";
    let query = sqlx::query_as::<_, Todo>(q).bind(id);
    let todo = query.fetch_optional(pool).await?;
    println!("{:?}", todo);
    Ok(())
}
pub async fn search_todo_by_name(name: String, pool: &PgPool) -> Result<()> {
    let q = "SELECT * FROM todos WHERE name LIKE $1";
    let query = sqlx::query_as::<_, Todo>(q).bind(name + "%");
    let todos = query.fetch_all(pool).await?;
    println!("{:?}", todos);
    Ok(())
}
