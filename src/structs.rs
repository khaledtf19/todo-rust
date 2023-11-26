use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub discription: String,
    pub done: bool
}
