use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors, ValidationError};

use crate::data::architect::Architect;


impl Architect {
    pub fn into_dto(self, token: String) -> ResponseArchitectDto {
        ResponseArchitectDto {
            id: self.id,
            handle: self.handle,
            access_token: Some(token),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ResponseArchitectDto {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: Uuid,
    pub handle: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ArchitectAuthenticationResponse {
    pub architect: ResponseArchitectDto,
}

impl ArchitectAuthenticationResponse {
    pub fn new(
        id: Uuid,
        handle: String,
        access_token: Option<String>,
    ) -> Self {
        ArchitectAuthenticationResponse {
            architect: ResponseArchitectDto {
                id,
                handle,
                access_token,
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct RegisterArchitectDto {
    #[validate(custom = "validate_handle")]
    pub handle: Option<String>,
    #[validate(required, length(min = 7))]
    pub password: Option<String>,
}



#[derive(Clone, Serialize, Deserialize, Debug, Validate, Default)]
pub struct ConnectArchitectDto {
    #[validate(custom = "validate_handle")]
    pub handle: Option<String>,
    #[validate(required, length(min = 7))]
    pub password: Option<String>,
}

fn validate_handle(handle: &str) -> Result<(), ValidationError> {
    // Your custom validation logic here
    if handle.chars().count() >= 3 {
        Ok(())
    } else {
        return Err(ValidationError::new("User handle is too short"));
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct UpdateArchitectDto {
    pub handle: Option<String>,
    pub password: Option<String>,
}


//--TODO: Stubbing, add later
// impl RegisterArchitectDto {
//     pub fn new_stub -> Self {
//         Self {}
//     }
// }

// impl ConnectArchitectDto {
//     pub fn new_stub -> Self {
//         Self {}
//     }
// }
