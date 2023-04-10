use crate::{
    model::product::{CreateProduct, Product},
    AppState,
};
use actix_web::web::Data;
use sqlx::Error;
use uuid::Uuid;

pub async fn get_product(state: Data<AppState>) -> Result<Vec<Product>, Error> {
    sqlx::query_as!(Product, "SELECT * FROM products")
        .fetch_all(&state.db)
        .await
}

pub async fn create_product(
    state: Data<AppState>,
    product_data: CreateProduct,
) -> Result<Product, Error> {
    let id = uuid::Uuid::new_v4();
    let CreateProduct {
        name,
        price,
        gym_id,
    } = product_data;

    let response = sqlx::query_as!(
        Product,
        "INSERT INTO Products (id, name, price, gym_id) VALUES ($1, $2, $3, $4) RETURNING id, name, price, gym_id",
        id,
        name,
        price, 
        gym_id
    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub async fn find_product_by_id(state: Data<AppState>, id: Uuid) -> Result<Product, Error>{
    sqlx::query_as!(Product, "SELECT * FROM products WHERE id = $1", id).fetch_one(&state.db).await
}

pub async fn find_products_by_gym_id(state: Data<AppState>, gym_id: Uuid) -> Result<Vec<Product>, Error>{
    sqlx::query_as!(Product, "SELECT * FROM products WHERE gym_id = $1", gym_id).fetch_all(&state.db).await
}
