use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Report {
    elf: usize,
    #[serde(rename = "elf on a shelf")]
    elf_on_a_shelf: usize,
    #[serde(rename = "shelf with no elf on it")]
    shelf_with_no_elf_on_it: usize,
}

pub fn task() -> Router {
    Router::new().route("/", post(count_elf))
}

async fn count_elf(body: String) -> Json<Report> {
    let elf_count = body.matches("elf").count();
    let elf_on_a_shelf_count = body.matches("elf on a shelf").count();
    let shelf_count = body.matches("shelf").count();
    let report = Report {
        elf: elf_count,
        elf_on_a_shelf: elf_on_a_shelf_count,
        shelf_with_no_elf_on_it: shelf_count - elf_on_a_shelf_count,
    };
    Json(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use serde_json::json;

    #[tokio::test]
    async fn task1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text(
                "The mischievous elf peeked out from behind the toy workshop,
        and another elf joined in the festive dance.
        Look, there is also an elf on that shelf!",
            )
            .await;

        response.assert_status(StatusCode::OK);

        assert_eq!(response.json::<Report>().elf, 4);
    }

    #[tokio::test]
    async fn task2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text(
                "there is an elf on a shelf on an elf.
                there is also another shelf in Belfast.",
            )
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
            "elf": 5,
            "elf on a shelf": 1,
            "shelf with no elf on it": 1
        }));
    }
}
