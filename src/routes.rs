use actix_web::{get, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

use crate::app::App;

#[get("/")]
async fn index(_data: web::Data<state::AppState>) -> HttpResponse {
    return html! {
        <App/>
    }
}