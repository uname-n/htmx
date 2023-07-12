use leptos::*;

use crate::component::counter::Counter;
use crate::component::todo::TodoForm;

#[component]
pub fn App(cx: Scope, counter: usize, todo_list: Vec<String>) -> impl IntoView {
    return view! {cx,
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
        </head>
        <body>
            <Counter count=counter/>
            <form hx-trigger="submit" hx-post="/counter/clicked" hx-target="#counter" hx-swap="outerHTML">
                <button>"click me"</button>
            </form>
            <TodoForm todo_list=todo_list/>
        </body>
    }
}
