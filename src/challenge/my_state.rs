#[derive(Clone)]
pub struct MyState {
    pub pool: sqlx::PgPool,
}
