mod models;
mod state;
mod handlers;

use handlers::update_todo;

use axum::{
    extract::{State, Json},
    routing::{get, post, put},
    Router,
};
use models::{NewTodo, Todo};
use state::SharedState;

use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let todos: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/", get(hello))
        .route("/todos", post(create_todo))
        .route("/get-todo", get(get_todos).post(create_todo))
        .route("/todos/:id", put(create_todo))
        .with_state(todos);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// GET /
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
