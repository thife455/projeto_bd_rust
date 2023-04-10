use actix_web::{
    get,
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    model::product::Product,
    repositories::{product::find_product_by_id, user_product::get_user_product_by_user_id},
    AppState,
};

pub fn user_product_scope() -> actix_web::Scope {
    web::scope("")
        .service(get_user_product_by_user_controller)
        .service(get_user_products_products_by_user_controller)
}

#[get("/user_products/user/{id}")]
async fn get_user_product_by_user_controller(
    state: Data<AppState>,
    info: web::Path<(uuid::Uuid,)>,
) -> impl Responder {
    let user_id = info.0;
    let user_response = get_user_product_by_user_id(state, user_id).await;
    match user_response {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[get("/user_products/user/{id}/products")]
async fn get_user_products_products_by_user_controller(
    state: Data<AppState>,
    info: web::Path<(uuid::Uuid,)>,
) -> impl Responder {
    let user_id = info.0;
    let user_response = get_user_product_by_user_id(state.clone(), user_id).await;
    match user_response {
        Ok(users) => {
            let mut products: Vec<Product> = vec![];
            for user in users {
                products.push(
                    find_product_by_id(state.clone(), user.product_id)
                        .await
                        .unwrap(),
                );
            }
            HttpResponse::Ok().json(products)
        }
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}
