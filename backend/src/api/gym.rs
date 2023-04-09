use crate::model::gym::*;
use crate::repositories::gym::*;
use crate::AppState;
use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Responder,
};

#[get("/gym")]
pub async fn get_gyms_controller(state: Data<AppState>) -> impl Responder {
    let gym_response = get_gyms(state).await;
    match gym_response {
        Ok(gyms) => HttpResponse::Ok().json(gyms),
        Err(_e) => HttpResponse::InternalServerError().json("Unexpected error in query"),
    }
}

#[post("/gym")]
pub async fn create_gym_controller(
    state: Data<AppState>,
    body: web::Json<CreateGym>,
) -> impl Responder {
    let params = body.into_inner();
    let response = create_gym(&state, params).await;
    match response {
        Ok(gym) => HttpResponse::Ok().json(gym),
        Err(_e) => HttpResponse::InternalServerError().json("Error in query"),
    }
}
