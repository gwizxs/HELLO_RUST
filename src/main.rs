mod api;

mod shared;
mod domain;
mod state;

use api::router::create_router;
use state::SharedState;

use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let todos: SharedState = Arc::new(Mutex::new(Vec::new()));

    let app = create_router(todos);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("🚀 listening on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
