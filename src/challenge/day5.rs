use axum::{extract::Query, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    #[serde(default)]
    offset: usize,
    limit: Option<usize>,
    split: Option<usize>,
}

pub fn task() -> Router {
    Router::new().route("/", post(list_names))
}

async fn list_names(
    pagination: Query<Pagination>,
    Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
    let pagination: Pagination = pagination.0;
    let offset = pagination.offset;
    let limit = pagination.limit.unwrap_or(names.len() - offset);

    let list = names
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect::<Vec<String>>();
    if let Some(split) = pagination.split {
        if split == 0 {
            (StatusCode::BAD_REQUEST, "split cannot be 0").into_response()
        } else {
            Json(list.chunks(split).collect::<Vec<&[String]>>()).into_response()
        }
    } else {
        Json(list).into_response()
    }
}
