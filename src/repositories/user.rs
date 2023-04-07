use crate::{
    model::user::{CreateUser, User},
    AppState,
};
use actix_web::web::Data;
use sqlx::{Error, FromRow};

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
        "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)",
        id,
        name,
        email
    )
    .fetch_one(&state.db)
    .await;

    match response {
        Ok(user) => User::from_row(&user),
        Err(e) => Err(e),
    }
}
