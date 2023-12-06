use axum::Router;

mod challenge;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .nest("/1", challenge::day1::task())
        .nest("/4", challenge::day4::task())
        .nest("/6", challenge::day6::task())
        .nest("/", challenge::day_1::task());

    Ok(router.into())
}
