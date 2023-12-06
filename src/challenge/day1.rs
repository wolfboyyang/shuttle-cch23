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
