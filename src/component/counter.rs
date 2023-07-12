use actix_web::{post, web, HttpResponse};
use leptos::*;

use crate::html;
use crate::state;

#[component]
pub fn Counter(cx: Scope, count:usize) -> impl IntoView {
    return view! {cx,
        <div id="counter">"Current Count: " {format!("{}", count)}</div>
    }
}

#[post("/counter/clicked")]
pub async fn clicked(data: web::Data<state::AppState>) -> HttpResponse {
    let counter = data.counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
    return html! { 
        <Counter count=counter/>
    }
}