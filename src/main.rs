use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Serdeer {
    name: String,
    strength: i32,
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
}

async fn cube_the_bits(Path(path): Path<String>) -> impl IntoResponse {
    let cube_bits = path
        .split('/')
        .map(|s| s.parse::<i32>().unwrap())
        .inspect(|x| println!("{}", x.to_string()))
        .fold(0, |acc, x| acc ^ x);
    let sled_id = cube_bits.pow(3);
    (StatusCode::OK, sled_id.to_string())
}

async fn cacalate_combined_strength(Json(payload): Json<Vec<Serdeer>>) -> impl IntoResponse {
    let strength = payload.iter().map(|x| x.strength).sum::<i32>();
    (StatusCode::OK, strength.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*path", get(cube_the_bits))
        .route("/4/strength", post(cacalate_combined_strength))
        .route("/-1/error", get(handle_error));

    Ok(router.into())
}
