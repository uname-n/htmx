use actix_web::{get, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

use crate::app::App;

#[get("/")]
async fn index(_data: web::Data<state::AppState>) -> HttpResponse {
    let host = std::env::var("HOST").unwrap_or("http://localhost:8080".to_string());
    return html! {
        <App host=host/>
    }
}