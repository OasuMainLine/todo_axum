use std::path::Path;

use crate::models::Todo;
use crate::models::TodoPayload;
use crate::utils::cwd;
use axum::extract;
use axum::http;
use axum::routing::delete;
use axum::routing::put;
use axum::routing::{get, post, Router};
use serde::Serialize;
use sqlx::sqlite::SqlitePool;
use uuid::Uuid;

#[derive(Serialize, Debug)]
struct Message {
    message: String,
}

impl Message {
    pub fn new(msg: String) -> Message {
        Self { message: msg }
    }
    pub fn new_json(msg: String) -> axum::Json<Message> {
        axum::Json(Self { message: msg })
    }
}

type MessageResponse = (http::StatusCode, axum::Json<Message>);
pub async fn get_router() -> Router {
    let file_path = cwd().join("db").join("todo.db");
    let connection =
        SqlitePool::connect(file_path.into_os_string().into_string().unwrap().as_str())
            .await
            .unwrap();

    let router = Router::new()
        .route("/", get(health))
        .route("/todos", get(get_todos))
        .route("/todos/:id", put(update_todo))
        .route("/todos/:id", get(get_todo_by_id))
        .route("/todos/:id", delete(delete_todo_by_id))
        .route("/todos/complete/:id", put(complete_todo))
        .route("/todos", post(create_todo))
        .with_state(connection);
    router
}

async fn health() -> MessageResponse {
    (
        http::StatusCode::OK,
        Message::new_json("Server running".to_string()),
    )
}

async fn create_todo(
    extract::State(pool): extract::State<SqlitePool>,
    axum::Json(payload): axum::Json<TodoPayload>,
) -> Result<(http::StatusCode, axum::Json<Todo>), http::StatusCode> {
    let todo = Todo::new(payload.name, payload.completed);
    let mut connection = pool.acquire().await.unwrap();
    let result = sqlx::query(
        r#"
    INSERT INTO todos (id, name, completed, inserted_at, updated_at) VALUES ($1, $2, $3, $4, $5)
    "#,
    )
    .bind(&todo.id)
    .bind(&todo.name)
    .bind(&todo.completed)
    .bind(&todo.inserted_at)
    .bind(&todo.updated_at)
    .execute(&mut *connection)
    .await;

    match result {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(todo))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
async fn get_todos(
    extract::State(pool): extract::State<SqlitePool>,
) -> Result<(http::StatusCode, axum::Json<Vec<Todo>>), http::StatusCode> {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&pool)
        .await;

    match result {
        Ok(todos) => Ok((http::StatusCode::OK, axum::Json(todos))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
async fn update_todo(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    axum::Json(payload): axum::Json<TodoPayload>,
) -> http::StatusCode {
    let mut connection = pool.acquire().await.unwrap();
    let now = chrono::Utc::now();
    let result = sqlx::query(
        r#"
        UPDATE todos SET name = $1, completed = $2, updated_at = $3 
        WHERE id = $4
    "#,
    )
    .bind(&payload.name)
    .bind(&payload.completed)
    .bind(&now)
    .bind(id)
    .execute(&mut *connection)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match result {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_todo_by_id(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<Uuid>,
) -> Result<axum::Json<Todo>, http::StatusCode> {
    let result = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(todo) => Ok(axum::Json(todo)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_todo_by_id(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<Uuid>,
) -> Result<MessageResponse, http::StatusCode> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(&id)
        .execute(&pool)
        .await
        .map(|res| match res.rows_affected() {
            0 => http::StatusCode::NOT_FOUND,
            _ => http::StatusCode::OK,
        });

    match result {
        Ok(status) => {
            let msg = match status {
                http::StatusCode::NOT_FOUND => Message::new_json("Record not found".to_string()),
                http::StatusCode::OK => {
                    Message::new_json(format!("Record {} deleted successfuly", id))
                }
                _ => panic!(),
            };
            Ok((status, msg))
        }
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn complete_todo(
    extract::State(pool): extract::State<SqlitePool>,
    extract::Path(id): extract::Path<Uuid>,
) -> Result<(http::StatusCode, axum::Json<Todo>), http::StatusCode> {
    let now = chrono::Utc::now();
    let result = sqlx::query_as::<_, Todo>(
        "UPDATE todos SET completed = 1, updated_at = $1 WHERE id = $2 RETURNING *",
    )
    .bind(now)
    .bind(&id)
    .fetch_optional(&pool)
    .await;

    match result {
        Ok(optional_todo) => match optional_todo {
            Some(todo) => Ok((http::StatusCode::OK, axum::Json(todo))),
            None => Err(http::StatusCode::NOT_FOUND),
        },
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
