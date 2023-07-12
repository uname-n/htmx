use leptos::*;

use crate::component::todo::TodoForm;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    return view! {cx,
        <head>
            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
        </head>
        <body>
            <TodoForm/>
        </body>
    }
}
