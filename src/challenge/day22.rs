use std::collections::HashMap;

use axum::{response::IntoResponse, routing::post, Router};

pub fn task() -> Router {
    Router::new().route("/integers", post(get_present))
}

async fn get_present(text: String) -> impl IntoResponse {
    let dict = text
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|num| num.trim().parse::<u64>().unwrap())
        .fold(HashMap::new(), |mut dict, num| {
            if dict.contains_key(&num) {
                dict.remove_entry(&num);
            } else {
                dict.insert(num, ());
            }
            dict
        });

    let ord_num = *dict.keys().next().unwrap();

    "🎁".repeat(ord_num as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;

    #[tokio::test]
    async fn task1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/4/8").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(1728.to_string());
    }

    #[tokio::test]
    async fn task2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("4/5/8/10").await;

        response.assert_status(StatusCode::OK);

        response.assert_text(27.to_string());
    }
}
