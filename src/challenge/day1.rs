use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn task() -> Router {
    Router::new().route("/*path", get(cube_the_bits))
}

async fn cube_the_bits(Path(path): Path<String>) -> impl IntoResponse {
    let cube_bits = path
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .fold(0, |acc, x| acc ^ x);
    let sled_id = cube_bits.pow(3);
    (StatusCode::OK, sled_id.to_string())
}
