use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize, Serialize)]
struct Report {
    result: String,
    reason: String,
}

pub fn task() -> Router {
    Router::new()
        .route("/nice", post(check_password))
        .route("/game", post(play_game))
}

async fn check_password(payload: String) -> impl IntoResponse {
    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload) {
        if let Some(input) = payload.get("input") {
            let text = input.as_str().unwrap();
            // Rule 1: must contain at least 3 vowels
            let vowels = Regex::new(r"(.*[aeiouy]){3,}").unwrap();
            // Rule 2: must contain at least one letter that appears twice in a row
            let twice = Regex::new(r"([a-z])\1").unwrap();
            // Rule 3: must not contain ab, cd, pq, or xy
            let blacklist = Regex::new(r"ab|cd|pq|xy").unwrap();
            return if vowels.is_match(text).unwrap()
                && twice.is_match(text).unwrap()
                && !blacklist.is_match(text).unwrap()
            {
                (StatusCode::OK, Json(serde_json::json!({"result": "nice"})))
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({"result": "naughty"})),
                )
            };
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(serde_json::json!("response body does not matter")),
    )
}

async fn play_game(payload: String) -> impl IntoResponse {
    if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&payload) {
        if let Some(input) = payload.get("input") {
            let text = input.as_str().unwrap();
            // Rule 1: must be at least 8 characters long
            if text.len() < 8 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "8 chars".to_string(),
                    }),
                );
            }
            // Rule 2: must contain uppercase letters, lowercase letters, and digits
            let rule_2 = Regex::new(r"(?=.*[A-Z])(?=.*[a-z])(?=.*\d).*").unwrap();
            if !rule_2.is_match(text).unwrap() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "more types of chars".to_string(),
                    }),
                );
            }
            // Rule 3: must contain at least 5 digits
            let rule_3 = Regex::new(r"(.*\d.*){5,}").unwrap();
            if !rule_3.is_match(text).unwrap() {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "55555".to_string(),
                    }),
                );
            }

            // Rule 4: all integers must add up to 2023
            let rule_4 = Regex::new(r"\d+").unwrap();
            if rule_4
                .find_iter(text)
                .map(|m| m.unwrap().as_str().parse::<i32>().unwrap())
                .sum::<i32>()
                != 2023
            {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "math is hard".to_string(),
                    }),
                );
            }

            // Rule 5: must contain the letters j, o, and y in that order
            let rule_5 = Regex::new(r"^([^joy]*)j([^joy]*)o([^joy]*)y([^joy]*)$").unwrap();
            if !rule_5.is_match(text).unwrap() {
                return (
                    StatusCode::NOT_ACCEPTABLE,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "not joyful enough".to_string(),
                    }),
                );
            }

            // Rule 6: must contain a letter that repeats with exactly one other letter between them
            let rule_6 = Regex::new(r"([a-zA-Z])\w\1").unwrap();
            if !rule_6.is_match(text).unwrap() {
                return (
                    StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "illegal: no sandwich".to_string(),
                    }),
                );
            }

            // Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
            let rule_7 = Regex::new(r"[\u{2980}-\u{2BFF}]").unwrap();
            if !rule_7.is_match(text).unwrap() {
                return (
                    StatusCode::RANGE_NOT_SATISFIABLE,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "outranged".to_string(),
                    }),
                );
            }

            // Rule 8: must contain at least one emoji

            if emojito::find_emoji(text).is_empty() {
                return (
                    StatusCode::UPGRADE_REQUIRED,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "😳".to_string(),
                    }),
                );
            }

            // Rule 9: the hexadecimal representation of the sha256 hash must end with an a
            // create a Sha256 object
            let mut hasher = Sha256::new();

            // write input message
            hasher.update(text);

            // read hash digest and consume hasher
            let result = hasher.finalize();
            if !hex::encode(result).ends_with('a') {
                return (
                    StatusCode::IM_A_TEAPOT,
                    Json(Report {
                        result: "naughty".to_string(),
                        reason: "not a coffee brewer".to_string(),
                    }),
                );
            }

            return (
                StatusCode::OK,
                Json(Report {
                    result: "nice".to_string(),
                    reason: "that's a nice password".to_string(),
                }),
            );
        }
    }

    (
        StatusCode::BAD_REQUEST,
        Json(Report {
            result: "naughty".to_string(),
            reason: "response body does not matter".to_string(),
        }),
    )
}
