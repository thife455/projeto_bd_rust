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
    user_products (id) {
        id -> Uuid,
        product_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
    }
}

diesel::table! {
    wallets (id) {
        id -> Uuid,
        balance -> Int4,
        user_id -> Uuid,
    }
}

diesel::joinable!(products -> gyms (gym_id));
diesel::joinable!(user_products -> products (product_id));
diesel::joinable!(user_products -> users (user_id));
diesel::joinable!(wallets -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    gyms,
    products,
    user_products,
    users,
    wallets,
);
