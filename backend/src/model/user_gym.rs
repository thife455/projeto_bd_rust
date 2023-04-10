use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(FromRow, Deserialize, Serialize)]
pub struct UserGym {
    pub id: Uuid,
    pub gym_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserGym {
    pub gym_id: Uuid,
    pub user_id: Uuid,
}
