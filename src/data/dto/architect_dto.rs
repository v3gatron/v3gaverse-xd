// use crate::data::models::architect::Architect;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Add axum-login crate.  I think it's fine to handle sessions/login etc here since i'm the only user.  This should go in services though
#[derive(Deserialize, Serialize)]
pub struct Profile {
    pub mood: String,
    pub age: String,
}

#[derive(Serialize, Deserialize)] // TODO: Do you need validate? It's only you after all...
pub struct ArchitectDto {
    pub id: Uuid,
    pub handle: String,
    pub hash_pwd: String,
    pub last_login: NaiveDateTime,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    //    pub profile: Option<Profile>,
}
// NOTE: Data Transfer Obects
#[derive(Clone, Serialize, Deserialize)]
pub struct ArchitectConnectDto {
    pub handle: String,
    pub hash_pwd: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreateArchitectDto {
    pub handle: String,
    pub hash_pwd: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl std::fmt::Debug for CreateArchitectDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CreateArchitectDto")
            .field("handle", &self.handle)
            .field("hash_pwd", &self.hash_pwd)
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("avatar_url", &self.avatar_url)
            .finish()
    }
}

pub struct ReadArchitectDto {
    pub id: Uuid,
    pub handle: String,
    pub hash_pwd: String,
    pub last_login: Option<NaiveDateTime>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
}

impl ReadArchitectDto {
    pub fn from(model: ArchitectDto) -> ReadArchitectDto {
        Self {
            id: model.id,
            handle: model.handle,
            hash_pwd: model.hash_pwd,
            last_login: Some(model.last_login),
            first_name: model.first_name,
            last_name: model.last_name,
            avatar_url: model.avatar_url,
            //profile: Option<Profile>,
        }
    }
}

impl std::fmt::Debug for ReadArchitectDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReadArchitectDto")
            .field("id", &self.id)
            .field("handle", &self.handle)
            .field("hash_pwd", &self.hash_pwd)
            .field("last_login", &self.last_login)
            .field("first_name", &self.first_name)
            .field("last_name", &self.last_name)
            .field("avatar_url", &self.avatar_url)
            .finish()
    }
}

// TODO: Add Update as well.
