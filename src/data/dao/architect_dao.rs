use crate::data::dto::architect_dto::{self, ArchitectDto, CreateArchitectDto};
use sqlx::PgPool;
use std::sync::Arc;

struct ArchitectDao {
    pool: Arc<PgPool>,
}

impl ArchitectDao {
    pub async fn create_architect_unless_exists(
        &self,
        dto: CreateArchitectDto,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"INSERT INTO architects (handle, password) VALUES ($1, $2)"#,
            dto.handle,
            dto.hash_pwd,
        )
        .fetch_one(self.pool.as_ref()) // TODO PgPool connection here via app state or maybe the arc above?, is that the best way?
        .await?;

        // check to see if architect exists in table
        // if not then create
        // there should be a way to limit number of rows in sql table =)
        Ok(())
    }
}
