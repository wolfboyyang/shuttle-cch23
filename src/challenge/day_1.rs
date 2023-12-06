use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn task() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(handle_error))
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
}
