use leptos::*;

#[component]
pub fn TodoForm(cx: Scope) -> impl IntoView {
    return view! {cx,
        <form>
            <input type="text" name="item" placeholder="add item"/>
            <button>"Add item"</button>
        </form>
        <Todos todo_list=Vec::new()/>
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