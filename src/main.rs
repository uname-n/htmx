mod util;
mod state;
mod routes;

mod app;
mod component;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use libsql_client::{Config, Client};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::new("file:////tmp/example.db").unwrap();
    let client = Arc::new(Client::from_config(config).await.unwrap());

    client.execute("
        CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY, 
            data TEXT NOT NULL, 
            completed INTEGER NOT NULL DEFAULT 0
        )
    ").await.unwrap();

    let app_state = web::Data::new(state::AppState {
        client,
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(routes::index)
            .service(component::todo::todo_get)
            .service(component::todo::todo_add)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}