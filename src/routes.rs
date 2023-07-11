use actix_web::{get, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

use crate::component::counter::Counter;

#[get("/")]
async fn index(data: web::Data<state::AppState>) -> HttpResponse {
    let counter = data.counter.load(std::sync::atomic::Ordering::Relaxed);
    return html! {
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
        </head>
        <body>
            <Counter count=counter/>
            <form hx-trigger="submit" hx-post="/clicked" hx-target="#counter" hx-swap="outerHTML">
                <button>"click me"</button>
            </form>
        </body>
    }
}