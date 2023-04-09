use crate::{model::gym::*, AppState};

use actix_web::web::Data;
use sqlx::Error;

pub async fn get_gyms(state: Data<AppState>) -> Result<Vec<Gym>, Error> {
    sqlx::query_as!(Gym, "SELECT * FROM gyms")
        .fetch_all(&state.db)
        .await
}

pub async fn create_gym(state: Data<AppState>, user_data: CreateGym) -> Result<Gym, Error> {
    let id = uuid::Uuid::new_v4();
    let CreateGym { name, address , city} = user_data;

    let response = sqlx::query_as!(
        Gym,
        "INSERT INTO gyms (id, name, address, city) VALUES ($1, $2, $3, $4) RETURNING id, name, address, city",
        id,
        name,
        address, 
        city

    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(gym) => Ok(gym),
        Err(e) => Err(e),
    }
}
