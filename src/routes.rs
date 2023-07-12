use actix_web::{get, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

use crate::app::App;

#[get("/")]
async fn index(data: web::Data<state::AppState>) -> HttpResponse {
    let counter = data.counter.load(std::sync::atomic::Ordering::Relaxed);
    let todo_list = data.todo_list.lock().unwrap().clone();
    return html! {
        <App counter=counter todo_list=todo_list/>
    }
}