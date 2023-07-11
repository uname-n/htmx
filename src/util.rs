#[macro_export]
macro_rules! html {
    ($($body:tt)*) => {{
        let html  = leptos::ssr::render_to_string(move |cx| view! { cx, $($body)* });
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    }};
}