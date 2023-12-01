use axum::{
    extract::Path,
    http::StatusCode, response::IntoResponse, routing::get, Router};

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
}

async fn cube_the_bits(Path((num1, num2)): Path<(i32, i32)>) -> impl IntoResponse {
    let result = (num1 ^ num2).pow(3);
    (StatusCode::OK, result.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/:num1/:num2", get(cube_the_bits))
        .route("/-1/error", get(handle_error));

    Ok(router.into())
}
