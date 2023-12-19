use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use super::db::{insert_orders, reset, MyState};

pub fn task(state: MyState) -> Router {
    Router::new()
        .route("/sql", get(simple_query))
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/orders/total", get(orders_total_quantity))
        .route("/orders/popular", get(orders_popular_gift))
        .with_state(state)
}

async fn simple_query(State(state): State<MyState>) -> String {
    let record = sqlx::query!(r#"select 20231213 as "id!""#)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    record.id.to_string()
}

async fn orders_total_quantity(State(state): State<MyState>) -> impl IntoResponse {
    if let Ok(record) = sqlx::query!(r#"SELECT SUM(quantity) as "total!" FROM orders"#)
        .fetch_one(&state.pool)
        .await
    {
        Json(serde_json::json!({"total": record.total}))
    } else {
        Json(serde_json::json!({"total": 0}))
    }
}

async fn orders_popular_gift(State(state): State<MyState>) -> impl IntoResponse {
    if let Ok(record) = sqlx::query!(
        r#"SELECT gift_name as "popular!", SUM(quantity) AS gift_count
            FROM orders
            GROUP BY gift_name
            ORDER BY gift_count DESC
            LIMIT 1"#
    )
    .fetch_one(&state.pool)
    .await
    {
        Json(serde_json::json!({"popular": record.popular}))
    } else {
        Json(serde_json::json!({"popular": null}))
    }
}
