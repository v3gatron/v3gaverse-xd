use std::sync::Arc;
use async_trait::async_trait;
use mockall::automock;
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

#[derive(Debug, FromRow)] 
pub struct Architect {
    pub id: Uuid,
    pub handle: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
}

impl Default for Architect {
    fn default() -> Self {
        Architect {
            id: Uuid::new_v4(),
            handle: "v3gajerusalem".to_owned(),
            password: "mockingpass".to_owned(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            last_login: Some(Utc::now().naive_utc()),
        }
    }
}

pub type DynArchitectRepository = Arc<dyn ArchitectRepository + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArchitectRepository {
    async fn create_architect(
        &self,
        id: Uuid,
        handle: &str,
        hash_password: &str,
    ) -> anyhow::Result<Architect>;

    async fn get_architect_by_handle(&self, handl: &str) -> anyhow::Result<Option<Architect>>;

    async fn update_architect(
        &self,
        id: Uuid,
        handle: String,
        password: String,
    ) -> anyhow::Result<Architect>;
}
