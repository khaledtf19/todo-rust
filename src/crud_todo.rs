use anyhow::Ok;
use anyhow::Result;
use sqlx::PgPool;
use tabled::{Table, settings::Style};

use crate::args::{DoneCommand, DoneIdCommand, DoneNameCommand, DoneType};
use crate::structs::Todo;

pub async fn get_all_todos(pool: &PgPool)-> Result<()> {
    let q = "SELECT * FROM todos;";
    let todos = sqlx::query_as::<_,Todo>(q).fetch_all(pool).await?;
    print_table(&todos);
    Ok(())
}

pub async fn create_todo(name: String, discription: String, pool: &PgPool) -> Result<()> {
    sqlx::query("INSERT INTO todos (name, discription) VALUES ($1, $2)")
        .bind(&name)
        .bind(&discription)
        .execute(pool)
        .await?;
    get_all_todos(pool).await?;
    Ok(())
}

pub async fn done_todo(done: DoneCommand, pool: &PgPool) -> Result<()> {
    match done.done {
        DoneType::Id(DoneIdCommand { id }) => {
            let q = "UPDATE todos SET done = NOT done  WHERE id = $1;";
            sqlx::query(q).bind(&id).execute(pool).await?;
            search_todo_by_id(&id, &pool).await?;
        }
        DoneType::Name(DoneNameCommand { name }) => {
            let q = "Update todos SET done = NOT done WHERE name = $1";
            sqlx::query(q).bind(&name).execute(pool).await?;
            search_todo_by_name(&name, &pool, false).await?;
        }
    }
    Ok(())
}

pub async fn search_todo_by_id(id: &i32, pool: &PgPool) -> Result<()> {
    let q = "SELECT * FROM todos WHERE id = $1;";
    let query = sqlx::query_as::<_, Todo>(q).bind(id);
    let todo = query.fetch_optional(pool).await?;
    match todo {
        Some(todo) => print_table(&vec![todo]),
        None => println!("failed to find todo with id: {}", id),
    }

    Ok(())
}

pub async fn search_todo_by_name(name: &str, pool: &PgPool, many: bool) -> Result<()> {
    let q = "SELECT * FROM todos WHERE name LIKE $1";
    if many {
        let query = sqlx::query_as::<_, Todo>(q).bind(name.to_string() + "%");
        let todos = query.fetch_all(pool).await?;
        print_table(&todos);
        return Ok(());
    }
    let query = sqlx::query_as::<_, Todo>(q).bind(name);
    let todo = query.fetch_optional(pool).await?;
    match todo {
        Some(todo) => print_table(&vec![todo]),
        None => println!("failed to find todo with name: {}", name),
    }
    Ok(())
}

pub async fn delete_todo_by_id(id: i32, pool: &PgPool) -> Result<()> {
    let query = "DELETE FROM todos WHERE id = $1;";
    sqlx::query(query).bind(id).execute(pool).await?;

    println!("todo with id {} has been deleted ", id);
    get_all_todos(pool).await?;
    Ok(())
}

pub fn print_table(todos: &Vec<Todo>) {
    let table = Table::new(todos).with(Style::modern()).to_string();
    println!("{}", table);
}
