use actix_web::{App, HttpServer, web};
use std::sync::{Arc, Mutex};
mod db;
mod handlers;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match db::init_db() {
        Ok(_) => println!("Database initialized successfully."),
        Err(e) => eprintln!("Error initializing the database: {}", e),
    }

    let db_connection = Arc::new(Mutex::new(db::init_db().expect("Failed to initialize the database.")));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_connection.clone()))
            .configure(routes::config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}