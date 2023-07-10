use actix_web::{get, web, App, HttpServer, Result as AwResult};
use maud::{DOCTYPE, html, Markup, PreEscaped};
use std::sync::atomic::{AtomicUsize};

struct AppState {
    counter: AtomicUsize
}

#[get("/")]
async fn index(_data: web::Data<AppState>) -> AwResult<Markup> {
    Ok(html! {
        html {
            (DOCTYPE)
            (PreEscaped("<script src=\"https://unpkg.com/htmx.org@1.9.2\"></script>"))
            body {
                h1 {
                    "Current Count: ";
                    span id="counting" hx-trigger="load" hx-get="/clicked" hx-target="this" hx-swap="innerHTML";
                }
                form hx-trigger="submit" hx-get="/clicked" hx-target="#counting" hx-swap="innerHTML" {
                    input type="text" name="input_value" value="Placeholder";
                    button { "click me" };
                }
            }
        }
    })
}

#[get("/clicked")]
async fn clicked(data: web::Data<AppState>) -> AwResult<Markup> {
    let counter = data.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    Ok(html! {
        span { (counter) }
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                counter: AtomicUsize::new(0)
            }))
            .service(index)
            .service(clicked)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
