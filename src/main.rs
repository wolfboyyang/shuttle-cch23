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
    speed: Option<f32>,
    height: Option<i32>,
    antler_width: Option<i32>,
    snow_magic_power: Option<i32>,
    #[allow(dead_code)]
    favorite_food: Option<String>,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eat_yesterday: Option<i32>,
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

async fn handle_cursed_candy_eating_contest(
    Json(payload): Json<Vec<Serdeer>>,
) -> impl IntoResponse {
    let fastest_deer = payload
        .iter()
        .max_by(|x, y| x.speed.partial_cmp(&y.speed).unwrap())
        .unwrap();
    let tallest_deer = payload.iter().max_by_key(|x| x.height).unwrap();
    let magician = payload.iter().max_by_key(|x| x.snow_magic_power).unwrap();
    let consumer = payload
        .iter()
        .max_by_key(|x| x.candies_eat_yesterday)
        .unwrap();
    let report = format!(
        r#"{{
  "fastest": "Speeding past the finish line with a strength of {} is {}",
  "tallest": "{} is standing tall with his {} cm wide antlers",
  "magician": "{} could blast you away with a snow magic power of {}",
  "consumer": "{} ate lots of candies, but also some grass"
}}"#,
        fastest_deer.strength,
        fastest_deer.name,
        tallest_deer.name,
        tallest_deer.antler_width.unwrap(),
        magician.name,
        magician.snow_magic_power.unwrap(),
        consumer.name
    );
    (StatusCode::OK, report)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/1/*path", get(cube_the_bits))
        .route("/4/strength", post(cacalate_combined_strength))
        .route("/4/contest", post(handle_cursed_candy_eating_contest))
        .route("/-1/error", get(handle_error));

    Ok(router.into())
}
