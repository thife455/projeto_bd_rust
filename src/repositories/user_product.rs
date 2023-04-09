use crate::model::user_product::*;
use crate::AppState;
use actix_web::web::Data;
use sqlx::Error;

pub async fn create_user_product(
    state: Data<AppState>,
    user_product_data: CreateUserProduct,
) -> Result<UserProduct, Error> {
    let id = uuid::Uuid::new_v4();
    let CreateUserProduct {
        product_id,
        user_id,
    } = user_product_data;

    let response = sqlx::query_as!(
        UserProduct,
        "INSERT INTO user_products (id, product_id, user_id) VALUES ($1, $2, $3) RETURNING id, product_id, user_id",
        id,
        product_id,
        user_id
    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}
