use std::sync::Arc;
use tracing::{error, info};
use uuid::Uuid;
u
use async_trait::async_trait;


use crate::{
    server::{
        dto::architect_dto::{ResponseArchitectDto, ConnectArchitectDto, RegisterArchitectDto, UpdateArchitectDto},
        api::app_error
    },
   // utils::{argon_utils::DynArgoUtil, jwt_utils::DynJwtUtil},
    data::architect::DynArchitectRepository,
};

pub type DynArchitectService = Arc<dyn ArchitectServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait ArchitectServiceTrait {
    async fn register_architect(&self, request: RegisterArchitectDto) -> Result<ResponseArchitectDto>;

    async fn connect_architect(&self, request: ConnectArchitectDto, user_agent: Option<String>,) -> Result<ResponseArchitectDto>; //--NOTE: pay attention here, in the example it also returned a string.
}

