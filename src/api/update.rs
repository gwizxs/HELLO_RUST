use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::domain::todo::{Todo, UpdateTodo};
use crate::state::SharedState;

pub async fn update_todo(
    Path(id): Path<Uuid>,
    State(state): State<SharedState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.lock().await;

    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == id) {
        if let Some(title) = payload.title {
            todo.title = title;
        }
        if let Some(completed) = payload.completed {
            todo.completed = completed;
        }

        Ok(Json(todo.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

