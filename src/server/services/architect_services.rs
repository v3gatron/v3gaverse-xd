use std::sync::Arc;
use mockall::automock;
use tracing::{error, info};
use uuid::Uuid;

use async_trait::async_trait;
use crate::error::Result;

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
    async fn register_architect(&self, request: RegisterArchitectDto) -> Result<ResponseArchitectDto>;
    async fn connect_architect(&self, request: ConnectArchitectDto, user_agent: Option<String>,) -> Result<ResponseArchitectDto>; 
    async fn get_architect(&self, id: Uuid) -> Result<ResponseArchitectDto>;
    async fn update_architect(&self, id: Uuid, req: UpdateArchitectDto) -> Result<ResponseArchitectDto>;
}

impl ArchitectService {
    pub fn new(
        repository: DynArchitectRepository,
    ) -> Self {Self {repository}}
    

    
}


#[async_trait]
impl ArchitectServiceTrait for ArchitectService {
    async fn register_architect(&self, request: RegisterArchitectDto) -> Result<ResponseArchitectDto> {
        let handle = request.handle.unwrap();
        let password = request.password.unwrap();

        // TODO: check for existing architect by id...
        // TODO: has password 
    }
}
