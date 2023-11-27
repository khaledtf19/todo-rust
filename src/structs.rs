use sqlx::FromRow;
use tabled::Tabled;

#[derive(Debug, FromRow, Tabled)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub discription: String,
    pub done: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
