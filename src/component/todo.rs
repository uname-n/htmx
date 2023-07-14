use actix_web::{get, post, web, HttpResponse};
use leptos::*;
use serde::Deserialize;
use libsql_client::{Statement, args};

use crate::html;
use crate::state;

#[derive(Clone, Deserialize)]
pub struct Todo {
    id: Option<i64>,
    data: String,
    completed: Option<bool>,
}

#[component]
fn TodoItem(cx: Scope, todo: Todo) -> impl IntoView {
    return view! {cx,
        <li id=todo.id>
            <span>{todo.data}</span>
        </li>
    }
}

#[component]
pub fn TodoForm(cx: Scope, host:String) -> impl IntoView {
    return view! {cx,
        <div class="container-fluid">
            <form 
                hx-trigger="submit" 
                hx-post=format!("{}/todo/add", host) 
                hx-target="#todo_list" 
                hx-swap="beforeend" 
                hx-ext="json-enc"
                >
                <div class="input-group mx-auto p-4">
                    <input type="text" class="form-control" name="data" placeholder="add item"/>
                    <button class="btn btn-primary">"Add"</button>
                </div>
            </form>
        </div>
        <Todos host=host/>
    }
}

#[component]
fn Todos(cx: Scope, host:String) -> impl IntoView {
    return view! {cx,
        <ul id="todo_list"
            hx-trigger="load"
            hx-get=format!("{}/todo/get", host) 
            hx-swap="innerHTML"
        />
    }
}

#[get("/todo/get")]
async fn todo_get(data: web::Data<state::AppState>) -> HttpResponse {
    let todos = data.client.execute(Statement::new("SELECT * FROM todo")).await.unwrap();
    
    let mut todo_item: Vec<Todo> = Vec::new();
    for row in todos.rows {
        let id: i64 = row.try_column("id").unwrap();
        let data: &str = row.try_column("data").unwrap();
        let completed: usize = row.try_column("completed").unwrap();

        let todo = Todo {
            id: Some(id),
            data: data.to_string(),
            completed: Some(completed == 1),
        };

        todo_item.push(todo);
    }
    
    return html! { 
        <For
            each=move || todo_item.clone()
            key=|todo| todo.id.unwrap()
            view=move |cx, todo| view! {cx, <TodoItem todo=todo.clone()/> }
        />
    }
}

#[post("/todo/add")]
async fn todo_add(req: web::Json<Todo>, data: web::Data<state::AppState>) -> HttpResponse {
    let result = data.client.execute(Statement::with_args(
        "INSERT INTO todo (data) VALUES (?)", args![&req.data]
    )).await.unwrap();

    let mut req = req.into_inner();
    req.id = result.last_insert_rowid;

    return html! { 
        <TodoItem todo=req/>
    }
}