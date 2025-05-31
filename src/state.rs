use tokio::sync::Mutex;
use std::sync::Arc;

use crate::domain::todo::Todo;

pub type SharedState = Arc<Mutex<Vec<Todo>>>;
