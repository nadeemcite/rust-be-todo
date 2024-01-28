// src/handlers.rs
// use actix_web::{web, HttpResponse};
// use crate::models::{Todo, TodoInput, MessageResponse};

use actix_web::{web, HttpResponse, Error};
use crate::db;
use crate::models::TodoInput;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub async fn get_todos(db: web::Data<Arc<Mutex<Connection>>>,) -> Result<HttpResponse, Error> {
    
    let conn = db.lock().unwrap();
    match db::get_todos(&conn) {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error fetching todos")),
    }
}

pub async fn create_todo(db: web::Data<Arc<Mutex<Connection>>>, todo: web::Json<TodoInput>) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::create_todo(&conn, &todo.into_inner()) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error creating todo")),
    }
}

pub async fn get_todo(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::get_todo(&conn, *todo_id) {
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
    match db::update_todo(&conn, *todo_id, &todo_input.into_inner()) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error updating todo")),
    }
}


pub async fn delete_todo(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();
    match db::delete_todo(&conn, *todo_id) {
        Ok(_) => Ok(HttpResponse::Ok().json("Todo deleted successfully")),
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error deleting todo")),
    }
}

pub async fn toggle_complete(
    db: web::Data<Arc<Mutex<Connection>>>,
    todo_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let conn = db.lock().unwrap();

    match db::toggle_complete(&conn, *todo_id) {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)), 
        Err(_) => Ok(HttpResponse::InternalServerError().json("Error toggling todo completion status")),
    }
}