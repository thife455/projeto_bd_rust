// @generated automatically by Diesel CLI.

diesel::table! {
    gyms (id) {
        id -> Uuid,
        name -> Varchar,
        address -> Varchar,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        name -> Text,
        price -> Int4,
        gym_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::joinable!(products -> gyms (gym_id));

diesel::allow_tables_to_appear_in_same_query!(
    gyms,
    products,
    users,
);
