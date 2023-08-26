use std::sync::Arc;
use mockall::automock;
use tracing::{error, info};
use uuid::Uuid;

use async_trait::async_trait;
use crate::error::Result;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use crate::{
    server::dto::architect_dto::{ResponseArchitectDto, ConnectArchitectDto, RegisterArchitectDto, UpdateArchitectDto},
   data::architect::DynArchitectRepository,
    // NOTE: I'd like to know why the rust boilerplate code can jump straight to user::DynUserRepository
};

pub type DynArchitectService = Arc<dyn ArchitectServiceTrait + Send + Sync>;

pub struct ArchitectService {
    repository: DynArchitectRepository,
}

#[automock]
#[async_trait]
pub trait ArchitectServiceTrait {
    async fn register_architect(&self, req: RegisterArchitectDto) -> Result<ResponseArchitectDto>;
    // async fn connect_architect(&self, request: ConnectArchitectDto, user_agent: Option<String>,) -> Result<ResponseArchitectDto>; 
    // async fn get_architect(&self, id: Uuid) -> Result<ResponseArchitectDto>;
    // async fn update_architect(&self, id: Uuid, req: UpdateArchitectDto) -> Result<ResponseArchitectDto>;
}

impl ArchitectService {
    pub fn new(
        repository: DynArchitectRepository,
    ) -> Self {Self {repository}}
    

    
}

#[async_trait]
impl ArchitectServiceTrait for ArchitectService {
    async fn register_architect(&self, req: RegisterArchitectDto) -> Result<ResponseArchitectDto> {
        let id = uuid::Uuid::new_v4();
        let handle = req.handle.unwrap();
        let password = req.password.unwrap();

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        // TODO: Ok so now you do have to take a look at the argon util and see what's different.
        // This is a good time to learn the whole process and go back to your errors
        // TODO: '?' couldn't conver the error to error::Error
        // NOTE: So here you could return a result or error early, this is where it may be better to write smaller functions like you do in clojure
        let password_hash = argon2.hash_password(&password, &salt)?.to_string();
        //let existing_architect = self.repository.get_architect_by_handle(&handle).await?;
        // if existing_architect.is_some() {
        //     error!("I already exist.", handle);
        //     return Ok(("User exists"));
        // }
        
        // TODO: check for existing architect by id...
        // TODO: has password 
        let created_architect = self.repository.create_architect(&id, &handle, &password_hash).await?;
        Ok(created_architect.into_dto(String::new()))
    }
}
