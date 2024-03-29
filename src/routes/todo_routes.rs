// src/routes.rs
use actix_web::web;
use crate::handlers::todo_handlers::{get_todos, create_todo, get_todo, update_todo, delete_todo, toggle_complete};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("/", web::get().to(get_todos))
            .route("/", web::post().to(create_todo))
            .route("/{id}", web::get().to(get_todo))
            .route("/{id}", web::put().to(update_todo))
            .route("/{id}", web::delete().to(delete_todo))
            .route("/{id}/toggle_complete", web::patch().to(toggle_complete)),
    );
}
