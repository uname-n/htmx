use leptos::*;

use crate::component::todo::TodoForm;

#[component]
pub fn App(cx: Scope, host:String) -> impl IntoView {
    return view! {cx,
        <head>
            <meta charset="utf-8"/>
            <meta name="viewport" content="width=device-width, initial-scale=1"/>

            <title>"Todo App"</title>

            <script src="https://unpkg.com/htmx.org@1.9.2"></script>
            <script src="https://unpkg.com/htmx.org/dist/ext/json-enc.js"></script>
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0/dist/css/bootstrap.min.css"/>
        </head>
        <body>
            <TodoForm host=host/>
        </body>
    }
}
