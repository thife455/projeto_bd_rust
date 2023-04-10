use crate::model::user_product::CreateUserProduct;
use crate::repositories::gym::find_gym_by_id;
use crate::repositories::product::*;
use crate::repositories::user_product::create_user_product;
use crate::AppState;
use crate::{model::product::*, services::wallet::debit};

use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/products")]
pub async fn get_products_controller(state: Data<AppState>) -> impl Responder {
    let product_response = get_product(state).await;
    match product_response {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[get("/products/most_sold")]
pub async fn get_most_sold_products_controller(state: Data<AppState>) -> impl Responder {
    let product_response = search_most_sold_products(state).await;
    match product_response {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("/products")]
pub async fn create_products_controller(
    state: Data<AppState>,
    body: web::Json<CreateProduct>,
) -> impl Responder {
    let params = body.into_inner();
    let response = create_product(state, params).await;
    match response {
        Ok(gym) => HttpResponse::Ok().json(gym),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
#[get("/products/{id}/gym")]
pub async fn get_product_gym_controller(
    state: Data<AppState>,
    info: web::Path<(uuid::Uuid,)>,
) -> impl Responder {
    let product_id = info.0;

    let product = find_product_by_id(state.clone(), product_id).await.unwrap();

    let gym_query = find_gym_by_id(state, product.gym_id).await;

    match gym_query {
        Ok(gym) => HttpResponse::Ok().json(gym),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}

#[post("/products/{id}/buy")]
pub async fn buy_product_controller(
    state: Data<AppState>,
    info: web::Path<(uuid::Uuid,)>,
    body: web::Json<BuyProduct>,
) -> impl Responder {
    let product = find_product_by_id(state.clone(), info.0).await.unwrap();
    let user_id = body.into_inner().user_id;

    if let Err(e) = debit(state.clone(), &product.price, user_id)
        .await
        .map_err(|_e| HttpResponse::InternalServerError().json("Internal Server Error"))
    {
        return e;
    }

    let user_product_data = CreateUserProduct {
        user_id,
        product_id: info.0,
    };

    let user_product = create_user_product(state, user_product_data).await;

    match user_product {
        Ok(user_product) => HttpResponse::Ok().json(user_product),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
