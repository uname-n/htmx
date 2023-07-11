use leptos::*;

use crate::component::counter::Counter;
use crate::component::todo::TodoForm;

#[component]
pub fn App(cx: Scope, counter: usize) -> impl IntoView {
    return view! {cx,
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
        </head>
        <body>
            <Counter count=counter/>
            <form hx-trigger="submit" hx-post="/clicked" hx-target="#counter" hx-swap="outerHTML">
                <button>"click me"</button>
            </form>
            <TodoForm todo_list=Vec::new()/>
        </body>
    }
}
