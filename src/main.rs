use actix_web::{get, post, web, App, HttpServer, HttpResponse};
use std::sync::atomic::AtomicUsize;
use leptos::*;

struct AppState {
    counter: AtomicUsize
}

#[component]
fn counter(cx: Scope, count:usize) -> impl IntoView {
    return view! {cx,
        <div id="counter">"Current Count: " {format!("{}", count)}</div>
    }
}

#[post("/clicked")]
async fn clicked(data: web::Data<AppState>) -> HttpResponse {
    let counter = data.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    let html  = leptos::ssr::render_to_string(move |cx| view! { cx, 
        <Counter count=counter/>
    });
    return HttpResponse::Ok() 
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let counter = data.counter.load(std::sync::atomic::Ordering::Relaxed);
    let html  = leptos::ssr::render_to_string(move |cx| view! { cx,
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.2"/>
        </head>
        <body>
            <Counter count=counter/>
            <form hx-trigger="submit" hx-post="/clicked" hx-target="#counter" hx-swap="outerHTML">
                <button>"click me"</button>
            </form>
        </body>
    });
    return HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
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
