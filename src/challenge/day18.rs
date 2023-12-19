use std::collections::BTreeMap;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use super::db::{MyState, reset, insert_orders, insert_regions};

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
