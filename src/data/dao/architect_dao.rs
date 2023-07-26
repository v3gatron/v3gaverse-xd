use sqlx::PgPool;
use crate::data::models::architect;
use crate::data::dto::architect_dto;


pub async fn create_architect_unless_exists() -> Architect { 

    // check to see if architect exists in table
   // if not then create
    // there should be a way to limit number of rows in sql table =) 
}

