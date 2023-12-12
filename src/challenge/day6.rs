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

fn check_elf_on_shelf(start: usize, text: &str) -> bool {
    let elf_on_shelf = "elf on a shelf";
    let len = elf_on_shelf.len();
    let end = start + len;
    if end > text.len() {
        return false;
    }
    &text[start..end] == elf_on_shelf
}

async fn count_elf(body: String) -> Json<Report> {
    let elf_indices = body.match_indices("elf");
    let mut elf_on_a_shelf_count = 0;
    let elf_count = elf_indices
        .inspect(|(i, _)| {
            if check_elf_on_shelf(*i, &body) {
                elf_on_a_shelf_count += 1;
            }
        })
        .count();
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

    #[tokio::test]
    async fn task2_1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.post("/").text("elf elf elf on a shelf").await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
            "elf": 4,
            "elf on a shelf": 1,
            "shelf with no elf on it": 0
        }));
    }

    #[tokio::test]
    async fn task2_2() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text("In Belfast I heard an elf on a shelf on a shelf on a ")
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
            "elf": 4,
            "elf on a shelf": 2,
            "shelf with no elf on it": 0
        }));
    }

    #[tokio::test]
    async fn task2_3() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server
            .post("/")
            .text("Somewhere in Belfast under a shelf store but above the shelf realm there's an elf on a shelf on a shelf on a shelf on a elf on a shelf on a shelf on a shelf on a shelf on a elf on a elf on a elf on a shelf on a ")
            .await;

        response.assert_status(StatusCode::OK);

        response.assert_json(&json!({
            "elf": 16,
            "elf on a shelf": 8,
            "shelf with no elf on it": 2
        }));
    }
}
