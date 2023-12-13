use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};

type SharedState = Arc<std::sync::RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    time_capsule: HashMap<String, Instant>,
}

pub fn task() -> Router {
    let shared_state = SharedState::default();

    Router::new()
        .route("/save/:data", post(save_data))
        .route("/load/:data", get(load_data))
        .with_state(shared_state)
}

async fn save_data(Path(data): Path<String>, State(state): State<SharedState>) {
    let time_capsule = &mut state.write().unwrap().time_capsule;
    time_capsule.insert(data, Instant::now());
}

async fn load_data(Path(data): Path<String>, State(state): State<SharedState>) -> String {
    let time_capsule = &state.read().unwrap().time_capsule;
    let time = time_capsule.get(&data).unwrap();
    (*time).elapsed().as_secs().to_string()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use axum::http::StatusCode;
    use axum_test::TestServer;
    use std::thread::sleep;

    #[tokio::test]
    async fn task1() {
        let app = task();

        // Run the application for testing.
        let server = TestServer::new(app).unwrap();

        // Send the request.
        let response = server.post("/save/packet20231212").await;
        response.assert_status(StatusCode::OK);

        sleep(Duration::from_secs(2));

        let response = server.get("/load/packet20231212").await;
        response.assert_status(StatusCode::OK);

        response.assert_text("2");

        sleep(Duration::from_secs(2));

        let response = server.get("/load/packet20231212").await;
        response.assert_status(StatusCode::OK);

        response.assert_text("4");

        let response = server.post("/save/packet20231212").await;
        response.assert_status(StatusCode::OK);

        let response = server.get("/load/packet20231212").await;
        response.assert_status(StatusCode::OK);

        response.assert_text("0");
    }
}
