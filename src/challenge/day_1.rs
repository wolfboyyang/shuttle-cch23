use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn task() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/-1/error", get(handle_error))
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

async fn handle_error() -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
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
        let response = server.get("/").await;

        response.assert_status(StatusCode::OK);

        response.assert_text("Hello, world!");
    }

    #[tokio::test]
    async fn task2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.get("/-1/error").await;

        response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
    }
}
