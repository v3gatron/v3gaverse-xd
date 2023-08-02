use sqlx::PgPool;



// TODO: so this will be taken care of now
pub struct AppState {
    pub db_conn: PgPool,
}

