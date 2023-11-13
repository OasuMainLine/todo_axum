use serde::{Deserialize, Serialize};
use sqlx::FromRow;
type DATETIME = chrono::DateTime<chrono::Utc>;
#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub name: String,
    pub completed: bool,
    pub inserted_at: DATETIME,
    pub updated_at: DATETIME,
}
impl Todo {
    pub fn new(name: String, completed: bool) -> Todo {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            name,
            completed,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TodoPayload {
    pub name: String,
    pub completed: bool,
}
