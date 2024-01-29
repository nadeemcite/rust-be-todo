// src/handlers.rs
// use actix_web::{web, HttpResponse};
// use crate::models::{Todo, TodoInput, MessageResponse};

use actix_web::{web, HttpResponse, Error};
use crate::db;
use crate::models::todos::TodoInput;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn get_todos(db: web::Data<Arc<Mutex<Connection>>>,) -> Result<HttpResponse, Error> {
    
    let conn = db.lock().unwrap();
    match db::todos::get_todos(&conn) {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error fetching todos")),
    }
}

pub async fn create_todo(db: web::Data<Arc<Mutex<Connection>>>, todo: web::Json<TodoInput>) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::todos::create_todo(&conn, &todo.into_inner()) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error creating todo")),
    }
}

pub async fn get_todo(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::todos::get_todo(&conn, *todo_id) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error fetching todo")),
    }
}

pub async fn update_todo(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
    todo_input: web::Json<TodoInput>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();
    match db::todos::update_todo(&conn, *todo_id, &todo_input.into_inner()) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error updating todo")),
    }
}


pub async fn delete_todo(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();
    match db::todos::delete_todo(&conn, *todo_id) {
        Ok(_) => Ok(HttpResponse::Ok().json("Todo deleted successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error deleting todo")),
    }
}

pub async fn toggle_complete(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::todos::toggle_complete(&conn, *todo_id) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)), 
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error toggling todo completion status")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, App, http};
    use std::sync::{Arc, Mutex};
    use rusqlite::{Connection, params};

    async fn prepare_db() -> Arc<Mutex<Connection>> {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            )",
            [],
        ).unwrap();
        conn.execute("INSERT INTO todos (title, completed) VALUES (?1, ?2)", params!["Test Todo", false]).unwrap();
        Arc::new(Mutex::new(conn))
    }

    #[actix_web::test]
    async fn test_get_todo() {
        let db_connection = prepare_db().await;
        let todo_id = 1;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db_connection.clone()))
                .route("/todos/{id}", web::get().to(get_todo))
        ).await;
        let req = test::TestRequest::get()
            .uri(&format!("/todos/{}", todo_id))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), http::StatusCode::OK);

        let body = test::read_body(resp).await;
        let body = std::str::from_utf8(&body).unwrap();
        assert!(body.contains("Test Todo"));
    }
}

