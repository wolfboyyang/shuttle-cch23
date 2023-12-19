use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct MyState {
    pub pool: sqlx::PgPool,
}

#[derive(Deserialize, Serialize)]
pub struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Region {
    id: i32,
    name: String,
}

pub async fn reset(State(state): State<MyState>) {
    sqlx::query!("DROP TABLE IF EXISTS orders")
        .execute(&state.pool)
        .await
        .unwrap();

    sqlx::query!("DROP TABLE IF EXISTS regions")
        .execute(&state.pool)
        .await
        .unwrap();

    sqlx::query!(
        "CREATE TABLE regions (
            id INT PRIMARY KEY,
            name VARCHAR(50)
        )"
    )
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

pub async fn insert_orders(State(state): State<MyState>, Json(data): Json<Vec<Order>>) {
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

pub async fn insert_regions(State(state): State<MyState>, Json(data): Json<Vec<Region>>) {
    for region in data {
        sqlx::query!(
            "INSERT INTO regions (id, name) VALUES ($1, $2)",
            region.id,
            region.name
        )
        .execute(&state.pool)
        .await
        .unwrap();
    }
}
