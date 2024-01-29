use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};
mod handlers;
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match db::todos::init_db() {
        Ok(_) => println!("Database initialized successfully."),
        Err(e) => eprintln!("Error initializing the database: {}", e),
    }

    let db_connection = Arc::new(Mutex::new(db::todos::init_db().expect("Failed to initialize the database.")));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .configure(routes::todo_routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}