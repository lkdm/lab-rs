use axum::routing::get;
use lab_axum::db::data::DATA;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

#[tokio::main]
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/todos/", get(get_todos));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// axum handler for any request that fails to match the router routes.
// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// axum handler for "GET /" which returns a string and causes axum to
/// immediately respond with status code `200 OK` and with the string.
pub async fn hello() -> String {
    "Hello, World!".to_string()
}

pub async fn get_todos() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut todos: Vec<_> = data.todos.values().collect();
        todos.sort_by(|a, b| a.title.cmp(&b.title));
        todos
            .iter()
            .map(|&todo| format!("<p>{}</p>\n", &todo.title))
            .collect::<String>()
    })
    .join()
    .unwrap()
    .into()
}
