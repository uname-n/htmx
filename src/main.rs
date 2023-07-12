mod util;
mod state;
mod routes;

mod app;
mod component;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(state::AppState::new()))
            .service(routes::index)
            .service(component::counter::clicked)
            .service(component::todo::todo_add)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}