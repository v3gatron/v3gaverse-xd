use axum::Json;
use axum::extract::State;
//use crate::data::dto::architect_dto::{self, CreateArchitectDto};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;
use crate::api::web_server::ApplicationState;
use crate::data::app_state::AppState;
use crate::error::CustomError;
use super::postgres_repository::PostgresRepository;

// TODO: Add axum-login crate.  I think it's fine to handle sessions/login etc here since i'm the only user.  This should go in services though
#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub mood: String,
    pub age: String,
}

#[derive(Serialize, FromRow)] // TODO: Do you need validate? It's only you after all...
pub struct Architect {
    pub id: i32,
    pub handle: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>, 
    // pub last_login: NaiveDateTime,
    // pub first_name: Option<String>,
    // pub last_name: Option<String>,
    // pub avatar_url: Option<String>,
    //    pub profile: Option<Profile>,
}

#[derive(Deserialize, FromRow)] // TODO: Do you need validate? It's only you after all...
pub struct NewArchitect {
    pub handle: String,
    pub password: String,
    pub created_at: Option<NaiveDateTime>,

    //pub last_login: NaiveDateTime,
    //pub first_name: Option<String>,
    //pub last_name: Option<String>,
    //pub avatar_url: Option<String>,
    //    pub profile: Option<Profile>,
}

// NOTE: Data Transfer Obects
#[derive(Clone, Serialize, Deserialize)]
pub struct ConnectArchitect {
    pub handle: String,
    pub hash_pwd: String,
}

impl std::fmt::Debug for NewArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NewArchitect")
            .field("handle", &self.handle)
            .field("password", &self.password)
            .field("created_at", &self.created_at)
         //   .field("first_name", &self.first_name)
         //   .field("last_name", &self.last_name)
          //  .field("avatar_url", &self.avatar_url)
            .finish()
    }
}

pub struct GetArchitect {
    pub id: i32,
    pub handle: String,
    pub password: String,
  //  pub last_login: Option<NaiveDateTime>,
  //  pub first_name: Option<String>,
  //  pub last_name: Option<String>,
  //  pub avatar_url: Option<String>,
}

impl GetArchitect {
    pub fn from(model: Architect) -> GetArchitect {
        Self {
            id: model.id,
            handle: model.handle,
            password: model.password,
            // last_login: Some(model.last_login),
            // first_name: model.first_name,
            // last_name: model.last_name,
            // avatar_url: model.avatar_url,
            
            //profile: Option<Profile>,
        }
    }
}

impl std::fmt::Debug for GetArchitect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GetArchitect")
            .field("id", &self.id)
            .field("handle", &self.handle)
            .field("password", &self.password)
            // .field("last_login", &self.last_login)
            // .field("first_name", &self.first_name)
            // .field("last_name", &self.last_name)
            // .field("avatar_url", &self.avatar_url)
            .finish()
    }
}

// TODO: Add Update as well.

// struct ArchitectDAO {
//     pool: Arc<PgPool>,
// }

impl NewArchitect {
    pub async fn create(State(postgres_repository): State<PostgresRepository>,
                        Json(new_architect): Json<NewArchitect>) -> Result<(), CustomError> {
        let architect = sqlx::query_as!(Architect,
                                        "INSERT INTO architects (handle, password, created_at) VALUES ($1, $2, $3) RETURNING *",
                                        new_architect.handle,
                                        new_architect.password,
                                        new_architect.created_at)            
            .fetch_one(&postgres_repository.db_pool)
            .await.unwrap();
    Ok(())
    }
}

// NOTE: Ok so now to test create_architect
