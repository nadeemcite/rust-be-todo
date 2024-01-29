use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TodoInput {
    pub title: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: &'static str,
}
