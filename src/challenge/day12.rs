use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, Datelike, Utc};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{collections::HashMap, sync::Arc};
use ulid::Ulid;
use uuid::Uuid;

type SharedState = Arc<std::sync::RwLock<AppState>>;

#[derive(Default)]
struct AppState {
    time_capsule: HashMap<String, Instant>,
}

#[derive(Serialize, Deserialize)]
struct Report {
    #[serde(rename = "christmas eve")]
    christmas_eve: u32,
    weekday: u32,
    #[serde(rename = "in the future")]
    in_future: u32,
    #[serde(rename = "LSB is 1")]
    lbs: u32,
}

pub fn task() -> Router {
    let shared_state = SharedState::default();

    Router::new()
        .route("/save/:data", post(save_data))
        .route("/load/:data", get(load_data))
        .route("/ulids", post(convert_ulids))
        .route("/ulids/:weekday", post(analyze_ulids))
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

// Convert all the ULIDs to UUIDs and return a new array but in reverse order.
async fn convert_ulids(data: Json<Vec<String>>) -> Json<Vec<String>> {
    let ids: Vec<String> = data
        .iter()
        .map(|id| Uuid::from(Ulid::from_string(id).unwrap()).to_string())
        .rev()
        .collect();
    Json(ids)
}

async fn analyze_ulids(Path(weekday): Path<u32>, data: Json<Vec<String>>) -> Json<Report> {
    let mut lbs_count = 0;
    let dates: Vec<DateTime<Utc>> = data
        .iter()
        .map(|id| Ulid::from_string(id).unwrap())
        .inspect(|ulid| lbs_count += (ulid.0 & 1) as u32)
        .map(|ulid| DateTime::<Utc>::from(ulid.datetime()))
        .rev()
        .collect();
    let mut christmas_eve_count = 0;
    let mut weekday_count = 0;
    let mut future_day_count = 0;
    for date in dates {
        if date.month() == 12 && date.day() == 24 {
            christmas_eve_count += 1;
        }
        if date.weekday().num_days_from_monday() == weekday {
            print!("{} ", date.weekday());
            weekday_count += 1;
        }
        if date > Utc::now() {
            future_day_count += 1;
        }
    }
    Json(Report {
        christmas_eve: christmas_eve_count,
        weekday: weekday_count,
        in_future: future_day_count,
        lbs: lbs_count,
    })
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
