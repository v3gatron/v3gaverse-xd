use chrono::NaiveDateTime;
use uuid::Uuid;

use super::postgres_repository::{self, PostgresRepository};

pub struct IotaBody<T> {
    pub iota: T,
}

//--NOTE: Now for these, you may very well need to return the ID's...
pub struct IotaResponse {
    pub id: Uuid,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
}

pub struct Iota {
    pub id: Uuid,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct NewIota {
    pub user_id: Uuid, 
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl NewIota {
    pub async fn create(
        State(postgres_repository): State<PostgresRepository>,
        Json(new_iota): Json<NewIota>,
    ) -> Result<(), Error> {
        let id = Uuid::new_v4();
        let iota = sqlx::query_as!(
            Iota,
            "INSERT INTO iota (id, content, created_at) VALUES ($1, $2, $3) RETURNING *",
            new_iota.id,
            new_iota.content,
            new_iota.created_at,
        )
    }
}
