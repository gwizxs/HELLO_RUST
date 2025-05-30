use tokio::sync::Mutex; 

use crate::models::Todo;
use std::sync::Arc;

pub type SharedState = Arc<Mutex<Vec<Todo>>>; 
