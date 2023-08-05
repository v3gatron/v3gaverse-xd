use anyhow::Error;
use axum::extract::State;
use axum::Json;
use super::postgres_repository::PostgresRepository;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

//--ArchitectBody
// Response/Request body 
#[derive(Serialize, Deserialize)]
pub struct ArchitectBody<T> {
    pub architect: T,
}

#[derive(Serialize, FromRow)] // TODO: Do you need validate? It's only you after all...
pub struct Architect {
    pub id: i32,
    pub handle: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, FromRow)] // TODO: Do you need validate? It's only you after all...
pub struct ArchitectResponse {
    pub handle: String,
}


#[derive(Deserialize, FromRow)] // TODO: Do you need validate? It's only you after all...
pub struct NewArchitect {
    pub handle: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,
}

impl std::fmt::Debug for NewArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewArchitect")
            .field("handle", &self.handle)
            .field("password", &self.password)
            .field("created_at", &self.created_at)
            .finish()
    }
}

impl std::fmt::Debug for Architect{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewArchitect")
            .field("handle", &self.handle)
            .finish()
    }
}


impl std::fmt::Debug for ArchitectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewArchitect")
            .field("handle", &self.handle)
            .finish()
    }
}


// NOTE: If push comes to shove you may be better off combining this and your handler code
impl NewArchitect {
    pub async fn create(
        State(postgres_repository): State<PostgresRepository>,
        Json(new_architect): Json<NewArchitect>,
    ) -> Result<(), Error> {
        let architect = sqlx::query_as!(
            Architect,
            "INSERT INTO architects (handle, password, created_at) VALUES ($1, $2, $3) RETURNING *",
            new_architect.handle,
            new_architect.password,
            new_architect.created_at
        )
        .fetch_one(&postgres_repository.db_pool) // NOTE: Come back here
        .await
        .unwrap();
        Ok(())
    }
}

//--TODO: Ok so now to test create_architect
//--TODO: authentication code...will come later and most likely elsewhere 
// #[derive(Clone, Serialize, Deserialize)]
// pub struct ConnectArchitect {
//     pub handle: String,
//     pub hash_pwd: String,
// }
// TODO: Add axum-login crate.  I think it's fine to handle sessions/login etc here since i'm the only user.  This should go in services though


