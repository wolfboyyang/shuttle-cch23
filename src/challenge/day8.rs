use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct QueryResponse {
    data: i32,
    pantry: Recipe,
}

type Recipe = std::collections::HashMap<String, i32>;

#[derive(Serialize, Deserialize)]
struct Report {
    cookies: i32,
    pantry: Recipe,
}

pub fn task() -> Router {
    let client = Client::new();

    Router::new()
        .route("/weight/:number", get(get_pokemon_weight))
        .route("/drop/:number", get(drop_pokemon))
        .with_state(client)
}

async fn get_weight(number: i32, client: Client) -> i64 {
    if let Ok(response) = client
        .post("https://graphqlpokemon.favware.tech/v8")
        .header("Content-Type", "application/json")
        .body(format!(
            r#"{{
                "query": "query GetPokemonByDexNumber($number: Int!) {{ getPokemonByDexNumber(number: $number) {{weight}}}}",
                "variables": {{"number": {}}}
              }}"#,
            number
        ))
        .send()
        .await
    {
            let data: Value = serde_json::from_str(&response.text().await.unwrap()).unwrap();
            let weight = data["data"]["getPokemonByDexNumber"]["weight"]
        .as_i64()
        .unwrap();
            return weight;
    }

    -1
}

async fn get_pokemon_weight(
    Path(number): Path<i32>,
    State(client): State<Client>,
) -> impl IntoResponse {
    let weight = get_weight(number, client).await;
    (StatusCode::OK, weight.to_string())
}

async fn drop_pokemon(Path(number): Path<i32>, State(client): State<Client>) -> impl IntoResponse {
    let weight = get_weight(number, client).await;

    let height: f64 = 10.0;
    let gravity: f64 = 9.825;
    let momentum = (gravity * height * 2.0).sqrt() * weight as f64;

    (StatusCode::OK, momentum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn task1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/weight/25").await;

        response.assert_status(StatusCode::OK);

        response.assert_text("6");
    }

    #[tokio::test]
    async fn task2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/drop/25").await;

        response.assert_status(StatusCode::OK);

        let moment = response.text().parse::<f64>().unwrap();

        assert_approx_eq!(moment, 84.10707461325713, 0.001);
    }
}
