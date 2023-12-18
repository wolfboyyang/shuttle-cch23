use axum::Router;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

mod challenge;

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = challenge::my_state::MyState { pool };

    let router = Router::new()
        .nest("/1", challenge::day1::task())
        .nest("/4", challenge::day4::task())
        .nest("/6", challenge::day6::task())
        .nest("/7", challenge::day7::task())
        .nest("/8", challenge::day8::task())
        .nest("/11", challenge::day11::task())
        .nest("/12", challenge::day12::task())
        .nest("/13", challenge::day13::task(state.clone()))
        .nest("/14", challenge::day14::task())
        .nest("/15", challenge::day15::task())
        .nest("/18", challenge::day18::task(state))
        .nest("/", challenge::day_1::task());

    Ok(router.into())
}
