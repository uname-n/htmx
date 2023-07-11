use actix_web::{get, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

use crate::app::App;

#[get("/")]
async fn index(data: web::Data<state::AppState>) -> HttpResponse {
    let counter = data.counter.load(std::sync::atomic::Ordering::Relaxed);
    return html! {
        <App counter=counter/>
    }
}