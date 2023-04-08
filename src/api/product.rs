use crate::model::product::*;
use crate::repositories::product::*;
use crate::AppState;
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
