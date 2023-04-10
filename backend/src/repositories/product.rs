use crate::{
    model::product::{CreateProduct, Product},
    AppState,
};
use actix_web::{web::Data, App};
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

pub async fn search_most_sold_products(state: Data<AppState>) -> Result<Vec<Product>, Error> {
    sqlx::query_as!(Product, "SELECT * FROM products P ORDER BY (SELECT COUNT(*) FROM user_products UP WHERE UP.product_id = P.id)").fetch_all(&state.db).await
}

pub async fn search_product_above_price(state: Data<AppState>, price: i32) -> Result<Vec<Product>, Error> {
    sqlx::query_as!(Product, "SELECT * FROM products P WHERE P.price > $1", price).fetch_all(&state.db).await
}

pub async fn list_products_in_city(state: Data<AppState>, city: String) -> Result<Vec<Product>, Error> {
    sqlx::query_as!(Product, "SELECT P.id, P.name, P.price, P.gym_id FROM products P, gyms G WHERE P.gym_id =G.id AND G.city = $1", city).fetch_all(&state.db).await
}

pub async fn search_product_by_name_order_value(state: Data<AppState>, name: String) -> Result<Vec<Product>, Error> {
    sqlx::query_as!(Product, "SELECT p.name, P.id, P.price, P.gym_id FROM products P WHERE P.name LIKE $1  ORDER BY P.price", name).fetch_all(&state.db).await
}

