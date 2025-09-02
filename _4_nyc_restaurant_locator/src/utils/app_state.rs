use axum::extract::FromRef;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub conn: PgPool
}

impl AppState {
    pub fn new(conn: PgPool) -> Self {
        Self { conn }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> PgPool {
        state.conn.clone()
    }
}