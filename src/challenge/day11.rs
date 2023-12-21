use axum::{extract::Multipart, routing::post, Router};
use tower_http::services::ServeDir;

pub fn task() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/red_pixels", post(activate_bull_mode))
}

async fn activate_bull_mode(mut multipart: Multipart) -> String {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        if name != "image" {
            continue;
        }
        let data = field.bytes().await.unwrap();

        return match lodepng::decode_memory(data, lodepng::ColorType::RGBA, 8) {
            Ok(lodepng::Image::RGBA(image)) => image
                .buffer
                .iter()
                .filter(|pixel| pixel.r as u16 > pixel.g as u16 + pixel.b as u16)
                .count()
                .to_string(),
            Ok(_) => "Decoded image, but it was not RGBA".into(),
            Err(reason) => format!("Could not load, because: {reason}"),
        };
    }
    "No image found".into()
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
        let response = server.get("/assets/decoration.png").await;

        response.assert_status(StatusCode::OK);

        assert!(response
            .headers()
            .get("content-type")
            .is_some_and(|v| v == "image/png"));
        assert!(response
            .headers()
            .get("content-length")
            .is_some_and(|v| v == "787297"));

        let bytes = response.as_bytes();
        const EXPECTED: &[u8] = include_bytes!("../../assets/decoration.png");

        assert_eq!(bytes, EXPECTED);
    }
}
