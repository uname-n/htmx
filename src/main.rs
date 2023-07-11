use actix_web::{web, App, HttpServer};
use std::sync::atomic::AtomicUsize;

mod util;
mod state;
mod routes;

mod counter;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(state::AppState {
                counter: AtomicUsize::new(0)
            }))
            .service(routes::index)
            .service(counter::clicked)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
