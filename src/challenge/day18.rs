use std::collections::BTreeMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use super::my_state::MyState;

#[derive(Deserialize, Serialize)]
struct Order {
    id: i32,
    region_id: i32,
    gift_name: String,
    quantity: i32,
}

#[derive(Deserialize, Serialize)]
struct Region {
    id: i32,
    name: String,
}

#[derive(Deserialize, Serialize)]
struct RegionGift {
    region_name: String,
    gift_name: String,
    gift_count: Option<i64>,
}

pub fn task(state: MyState) -> Router {
    Router::new()
        .route("/reset", post(reset))
        .route("/orders", post(insert_orders))
        .route("/regions", post(insert_regions))
        .route("/regions/total", get(regions_total_quantity))
        .route("/regions/top_list/:num", get(regions_toplist))
        .with_state(state)
}

async fn reset(State(state): State<MyState>) {
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

async fn insert_regions(State(state): State<MyState>, Json(data): Json<Vec<Region>>) {
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

async fn regions_total_quantity(State(state): State<MyState>) -> impl IntoResponse {
    if let Ok(records) = sqlx::query!(
        r#"SELECT regions.name as "region_name!", SUM(orders.quantity) as "total!" FROM orders
    INNER JOIN regions ON orders.region_id=regions.id GROUP BY regions.name ORDER BY regions.name"#
    )
    .fetch_all(&state.pool)
    .await
    {
        Json(serde_json::json!(records
            .iter()
            .map(|r| {
                serde_json::json!({
                    "region": r.region_name,
                    "total": r.total})
            })
            .collect::<Vec<_>>()))
    } else {
        Json(serde_json::json!({"region": null, "total": 0}))
    }
}

async fn regions_toplist(
    Path(num): Path<usize>,
    State(state): State<MyState>,
) -> impl IntoResponse {
    let mut toplist = BTreeMap::<String, Vec<RegionGift>>::new();
    if let Ok(records) = sqlx::query!(r#"SELECT DISTINCT name as "region_name!" from regions"#)
        .fetch_all(&state.pool)
        .await
    {
        for record in records {
            toplist.insert(record.region_name, Vec::new());
        }
    }

    if let Ok(records) = sqlx::query_as!(RegionGift,
        r#"SELECT regions.name as "region_name!", orders.gift_name as "gift_name!", SUM(orders.quantity) as gift_count FROM regions
    INNER JOIN orders ON orders.region_id=regions.id GROUP BY regions.name, orders.gift_name ORDER BY regions.name, gift_count DESC, gift_name"#
    )
    .fetch_all(&state.pool)
    .await
    {
        for record in records {
            if let Some(list) = toplist.get_mut(&record.region_name) {
                if list.len() < num {
                    list.push(record);
                }
            }
        }
        
        Json(serde_json::json!(
            toplist
            .iter()
            .map(|(key, list)| {
                serde_json::json!({
                    "region": key,
                    "top_gifts": list.iter().map(|r| r.gift_name.clone()).collect::<Vec<_>>()})
            })
            .collect::<Vec<_>>()))
    } else {
        Json(serde_json::json!({"region": null, "top_gifts": null}))
    }
}
