use crate::{
    model::user::{CreateUser, User},
    AppState,
};
use actix_web::web::Data;
use sqlx::Error;
use uuid::Uuid;

pub async fn get_users(state: Data<AppState>) -> Result<Vec<User>, Error> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.db)
        .await
}

pub async fn create_user(state: Data<AppState>, user_data: CreateUser) -> Result<User, Error> {
    let id = uuid::Uuid::new_v4();
    let CreateUser { name, email } = user_data;

    let response = sqlx::query_as!(
        User,
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) RETURNING id, name, email",
        id,
        name,
        email
    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(user) => Ok(user),
        Err(e) => Err(e),
    }
}

pub async fn get_user_by_id(state: Data<AppState>, user_id: Uuid) -> Result<User, Error> {
    sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&state.db)
        .await
}

pub async fn find_user_by_name(state: Data<AppState>, name: String) -> Result<User, Error> {
    sqlx::query_as!(User, "SELECT * FROM users U WHERE U.name LIKE $1", name)
        .fetch_one(&state.db)
        .await
}

// pub async fn get_user(state: Data<AppState>, id: String) -> Result<User, Error> {
//     let id = id;
//     sqlx::query_as!(User, "SELECT * FROM users WHERE id = \"$1\";", id)
//         .fetch_one(&state.db)
//         .await
// }
