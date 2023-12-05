use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

mod challenge;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .nest("/1", challenge::day1::task_router())
        .nest("/4", challenge::day4::task_router())
        .route("/-1/error", get(handle_error));

    Ok(router.into())
}
