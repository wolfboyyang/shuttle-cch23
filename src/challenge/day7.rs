use axum::{routing::get, Json, Router};
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

type Recipe = std::collections::HashMap<String, i32>;

#[derive(Serialize, Deserialize)]
struct Kitchen {
    recipe: Recipe,
    pantry: Recipe,
}

#[derive(Serialize, Deserialize)]
struct Report {
    cookies: i32,
    pantry: Recipe,
}

pub fn task() -> Router {
    Router::new()
        .route("/decode", get(decode_cookie))
        .route("/bake", get(bake_cookie))
}

async fn decode_cookie(jar: CookieJar) -> Json<Recipe> {
    let encoded_recipe = jar.get("recipe").unwrap();
    let decoded_recipe = general_purpose::STANDARD
        .decode(encoded_recipe.value())
        .unwrap();
    let recipe = serde_json::from_slice::<Recipe>(&decoded_recipe).unwrap();

    Json(recipe)
}

async fn bake_cookie(jar: CookieJar) -> Json<Report> {
    let encoded_recipe = jar.get("recipe").unwrap();
    let decoded_recipe = general_purpose::STANDARD
        .decode(encoded_recipe.value())
        .unwrap();
    let mut kitchen = serde_json::from_slice::<Kitchen>(&decoded_recipe).unwrap();

    let cookies = kitchen
        .pantry
        .iter()
        .map(|(ingredient, amount_in_store)| {
            if let Some(amount_needed) = kitchen.recipe.get(ingredient) {
                amount_in_store / amount_needed
            } else {
                0
            }
        })
        .min()
        .unwrap_or(0);

    for (ingredient, amount_in_store) in kitchen.pantry.iter_mut() {
        if let Some(amount_needed) = kitchen.recipe.get(ingredient) {
            *amount_in_store -= amount_needed * cookies
        }
    }
    Json(Report {
        cookies,
        pantry: kitchen.pantry,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_extra::extract::cookie::Cookie;
    use axum_test::TestServer;

    #[tokio::test]
    async fn task1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/decode")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        let recipe = response.json::<Recipe>();

        assert_eq!(*recipe.get("flour").unwrap_or(&0), 100);
        assert_eq!(*recipe.get("chocolate chips").unwrap_or(&0), 20);
    }

    #[tokio::test]
    async fn task2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/bake")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        let recipe = response.json::<Report>();

        assert_eq!(recipe.cookies, 4);
        assert_eq!(*recipe.pantry.get("flour").unwrap_or(&0), 5);
        assert_eq!(*recipe.pantry.get("sugar").unwrap_or(&0), 307);
        assert_eq!(*recipe.pantry.get("butter").unwrap_or(&0), 2002);
        assert_eq!(*recipe.pantry.get("baking powder").unwrap_or(&0), 825);
        assert_eq!(*recipe.pantry.get("chocolate chips").unwrap_or(&0), 257);
    }

    #[tokio::test]
    async fn task3() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .get("/bake")
            .add_cookie(Cookie::new(
                "recipe",
                "eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==",
            ))
            .await;

        response.assert_status(StatusCode::OK);

        let recipe = response.json::<Report>();

        assert_eq!(recipe.cookies, 0);
        assert_eq!(*recipe.pantry.get("cobblestone").unwrap_or(&0), 64);
        assert_eq!(*recipe.pantry.get("stick").unwrap_or(&0), 4);
    }
}
