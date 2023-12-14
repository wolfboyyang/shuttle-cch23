use axum::{
    extract::State,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct MyState {
    pub pool: PgPool,
}

#[derive(Deserialize, Serialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

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

async fn reset(State(state): State<MyState>) {
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .unwrap();

    sqlx::query!(
        "CREATE TABLE orders (
        id INT PRIMARY KEY,
        region_id INT,
        gift_name VARCHAR(50),
        quantity INT
      )"
    )
    .execute(&state.pool)
    .await
    .unwrap();
}

async fn insert_orders(State(state): State<MyState>, Json(data): Json<Vec<Order>>) {
    for order in data {
        sqlx::query!(
            "INSERT INTO orders (id, region_id, gift_name, quantity) VALUES ($1, $2, $3, $4)",
            order.id,
            order.region_id,
            order.gift_name,
            order.quantity,
        )
        .execute(&state.pool)
        .await
        .unwrap();
    }
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
