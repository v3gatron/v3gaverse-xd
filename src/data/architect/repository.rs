use anyhow::Context;
use async_trait::async_trait;
//use chrono::Utc;
use sqlx::query_as;
use uuid::Uuid;

use crate::data::postgres_repository::PostgresRepository;
use super::{Architect, ArchitectRepository};

#[async_trait]
impl ArchitectRepository for PostgresRepository {
    async fn create_architect(&self, id: Uuid, handle: &str, hash_password: &str) -> anyhow::Result<Architect> {
        query_as!(
            Architect,
        r#"
        INSERT INTO architects (id, handle, password)
        VALUES ($1, $2, $3 ) RETURNING * "#, 
        id,
        handle,
        hash_password,
    )
    .fetch_one(&self.db_pool)
    .await
    .context("an unexpected error occurred while creating an architect")
 }

    async fn get_architect_by_handle(&self, handle: &str) -> anyhow::Result<Option<Architect>> {
        query_as!(
            Architect,
            r#" select * from architects where handle = $1::varchar"#,
            handle,
        )
        .fetch_optional(&self.db_pool)
        .await
        .context("unexpected error while querying for user")
        
    }
    
    async fn update_architect(&self,
                              id: Uuid,
                              handle: String,
                              password: String,) -> anyhow::Result<Architect> {
        query_as!(Architect,
                  r#"
        update architects
        set
           handle = $1::varchar,
           password = $2::varchar,
           updated_at = current_timestamp
        where id = $3
        returning * "#,
        handle,
        password,
        id)
            .fetch_one(&self.db_pool)
            .await
            .context("could not update...")
    }
}
