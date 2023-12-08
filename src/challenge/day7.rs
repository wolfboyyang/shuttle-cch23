use axum::{routing::get, Json, Router};
use axum_extra::extract::cookie::CookieJar;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
struct Recipe {
    #[serde(default)]
    flour: i32,

    #[serde(default)]
    sugar: i32,

    #[serde(default)]
    butter: i32,

    #[serde(default)]
    #[serde(rename = "baking powder")]
    baking_powder: i32,

    #[serde(default)]
    #[serde(rename = "chocolate chips")]
    chocolate_chips: i32,
}

impl Recipe {
    fn bake(&mut self, recipe: &Self) -> Result<&str, &str> {
        if self.flour >= recipe.flour
            && self.sugar >= recipe.sugar
            && self.butter >= recipe.butter
            && self.baking_powder >= recipe.baking_powder
            && self.chocolate_chips >= recipe.chocolate_chips
        {
            self.flour -= recipe.flour;
            self.sugar -= recipe.sugar;
            self.butter -= recipe.butter;
            self.baking_powder -= recipe.baking_powder;
            self.chocolate_chips -= recipe.chocolate_chips;

            Ok("Done")
        } else {
            Err("Not enough ingredients!")
        }
    }
}

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
    let mut cookies = 0;
    while kitchen.pantry.bake(&kitchen.recipe).is_ok() {
        cookies += 1;
    }
    //Json(report)
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

        assert_eq!(recipe.flour, 100);
        assert_eq!(recipe.chocolate_chips, 20);
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
        assert_eq!(
            recipe.pantry,
            Recipe {
                flour: 5,
                sugar: 307,
                butter: 2002,
                baking_powder: 825,
                chocolate_chips: 257,
            }
        );
    }
}
