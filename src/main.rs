use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

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

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*path", get(cube_the_bits))
        .route("/-1/error", get(handle_error));

    Ok(router.into())
}
