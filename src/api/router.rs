use axum::{
    routing::{get, post, put},
    Router,
};

use crate::{
    api::update::update_todo,
    domain::todo::{NewTodo, Todo},
    state::SharedState,
};
use crate::shared::routes::Route;
use axum::extract::{State, Json};
use uuid::Uuid;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .route(Route::Root.path(), get(hello))
        .route(Route::Todos.path(), post(create_todo))
        .route(Route::GetTodo.path(), get(get_todos).post(create_todo))
        .route(Route::TodoById.path(), put(update_todo))
        .with_state(state)
}

async fn hello() -> &'static str {
    "Hello world!"
}

async fn get_todos(State(state): State<SharedState>) -> Json<Vec<Todo>> {
    let todos = state.lock().await;
    Json(todos.clone())
}

async fn create_todo(
    State(state): State<SharedState>,
    Json(payload): Json<NewTodo>,
) -> Json<Todo> {
    let todo = Todo {
        id: Uuid::new_v4(),
        title: payload.title,
        completed: false,
    };

    let mut todos = state.lock().await;
    todos.push(todo.clone());

    Json(todo)
}
