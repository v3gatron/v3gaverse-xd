// use crate::data::dto::architect_dto::{self, CreateArchitectDto};
// use sqlx::PgPool;
// use std::{path::PrefixComponent, sync::Arc};

// struct ArchitectDao {
//     pool: Arc<PgPool>,
// }

// impl ArchitectDao {
//     pub async fn create_architect(&self, dto: CreateArchitectDto) -> Result<(), sqlx::Error> {
//         sqlx::query!(
//             r#"INSERT INTO architects (handle, password) VALUES ($1, $2)"#,
//             dto.handle,
//             dto.hash_pwd,
//         )
//         // TODO PgPool connection here via app state or maybe the arc above?, is that the best way?
//         .fetch_one(self.pool.as_ref())
//         .await?;
//         /* TODO: check to see if architect exists in table,if not then create
//         there should be a way to limit number of rows in sql table =)
//           NOTE: Is it really needed?*/
//         Ok(())
//     }
// }
