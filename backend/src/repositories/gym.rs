use crate::{model::{gym::*, gym_product_query1::GymProductQuery1}, AppState};


use actix_web::web::Data;
use sqlx::Error;
use uuid::Uuid;

pub async fn get_gyms(state: Data<AppState>) -> Result<Vec<Gym>, Error> {
    sqlx::query_as!(Gym, "SELECT * FROM gyms")
        .fetch_all(&state.db)
        .await
}

pub async fn create_gym(state: &Data<AppState>, user_data: CreateGym) -> Result<Gym, Error> {
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


pub async fn find_gym_by_id(state: Data<AppState>, id: Uuid) -> Result<Gym, Error>{
    sqlx::query_as!(Gym, "SELECT * FROM gyms WHERE id = $1", id).fetch_one(&state.db).await
}

pub async fn find_gym_by_city(state: Data<AppState>, city: String) -> Result<Vec<Gym>, Error> {
    sqlx::query_as!(Gym, "SELECT * FROM gyms G WHERE g.city = $1", city).fetch_all(&state.db).await
}

pub async fn find_gym_by_name(state: Data<AppState>, name: String) -> Result<Vec<Gym>, Error> {
    sqlx::query_as!(Gym, "SELECT * FROM gyms G WHERE G.name LIKE $1", name).fetch_all(&state.db).await
}

pub async fn order_gym_by_sale(state: Data<AppState>) -> Result<Vec<Gym>, Error> {
    sqlx::query_as!(Gym, "SELECT * FROM gyms G ORDER BY (SELECT COUNT (*) FROM products P, user_products UP WHERE P.gym_id = G.id AND UP.product_id = P.id)").fetch_all(&state.db).await
}

pub async fn find_products_in_gym(state: Data<AppState>, id: Uuid) -> Result<Vec<GymProductQuery1>, Error> {
    sqlx::query_as!(GymProductQuery1, "SELECT G.name as gym_name, P.name as product_name, P.price as price  FROM gyms G, products P WHERE G.id = P.gym_id AND G.id = $1", id).fetch_all(&state.db).await
}



