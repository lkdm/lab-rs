use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use lab_axum::db::data::DATA;
use lab_axum::model::todo::{Todo, TodoCreationDto};
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

/// Use Thread for spawning a thread e.g. to acquire our crate::DATA mutex lock.
use std::thread;

#[tokio::main]
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: Create connection pool here, to pass into app.

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello))
        .route("/todos/", get(get_todos).post(post_todos));

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
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

pub async fn get_todos() -> axum::response::Html<String> {
    // TODO: This does not need to be a thread
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

#[axum::debug_handler]
pub async fn post_todos(
    Json(todo): Json<TodoCreationDto>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let id = Uuid::new_v4();
    let mut data = DATA.lock().unwrap();
    let new_todo = Todo {
        id,
        title: todo.title,
        completed_at: None,
        archived_at: None,
    };
    // TODO: This Hashmap will always return an error for some reason.
    let _ = data
        .todos
        .insert(id, new_todo.clone())
        .ok_or_else(|| ApiError::CouldNotCreate)?;

    Ok::<StatusCode, ApiError>(StatusCode::CREATED)
}

impl IntoResponse for ApiError {
    // Useful for turning our ApiError into a HTTP response.
    fn into_response(self) -> Response {
        // Turns ApiError into a HTTP response.
        let (status, error_message) = match self {
            ApiError::InvalidUuid => (StatusCode::BAD_REQUEST, "Invalid UUID provided"),
            ApiError::CouldNotCreate => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Could not create todo")
            }
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Debug)]
enum ApiError {
    // A Custom Error type.
    InvalidUuid,
    CouldNotCreate,
}
