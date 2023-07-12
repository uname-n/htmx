use actix_web::{post, web, HttpResponse};
use leptos::*;
use serde::Deserialize;

use crate::html;
use crate::state;

#[component]
pub fn TodoForm(cx: Scope, todo_list: Vec<String>) -> impl IntoView {
    return view! {cx,
        <form hx-trigger="submit" hx-post="/todo/add" hx-target="#todo_list" hx-swap="outerHTML" hx-ext="json-enc">
            <input type="text" name="item" placeholder="add item"/>
            <button>"Add item"</button>
        </form>
        <Todos todo_list=todo_list/>
    }
}

#[component]
pub fn Todos(cx: Scope, todo_list: Vec<String>) -> impl IntoView {
    return view! {cx,
        <ul id="todo_list">
            {todo_list.into_iter()
                .map(|n| view! { cx, <li>{n}</li>})
                .collect_view(cx)}
        </ul>
    }
}

#[derive(Deserialize)]
struct TodoAdd {
    item: String,
}

#[post("/todo/add")]
async fn todo_add(req: web::Json<TodoAdd>, data: web::Data<state::AppState>) -> HttpResponse {
    let mut todo_list = data.todo_list.lock().unwrap();
    todo_list.push(req.item.clone());

    let new_todo_list = todo_list.clone();

    return html! { 
        <Todos todo_list=new_todo_list/>
    }
}