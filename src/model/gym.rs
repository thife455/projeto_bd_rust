use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Deserialize, Serialize)]
struct Gym {
    id: String,
    name: String,
    address: String,
    price: i32,
}

