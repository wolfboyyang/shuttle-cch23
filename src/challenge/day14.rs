use axum::{response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct HtmlContent {
    content: String,
}

macro_rules! html_template {
    () => {
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#
    };
}

pub fn task() -> Router {
    Router::new()
        .route("/unsafe", post(render_unsafe_html))
        .route("/safe", post(render_safe_html))
}

async fn render_unsafe_html(Json(payload): Json<HtmlContent>) -> impl IntoResponse {
    format!(html_template!(), payload.content)
}

async fn render_safe_html(Json(payload): Json<HtmlContent>) -> impl IntoResponse {
    format!(
        html_template!(),
        payload
            .content
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    )
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
