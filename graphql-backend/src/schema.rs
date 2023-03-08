table! {
    assets (id) {
        id -> Varchar,
        title -> Text,
        description -> Text,
        image_url -> Text,
        meta_data -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    followers (user_id, follower_id) {
        user_id -> Uuid,
        follower_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    portfolios (user_id, lego_set_id) {
        user_id -> Uuid,
        lego_set_id -> Varchar,
        quantity -> Int4,
        condition -> Varchar,
        meta_data -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(portfolios -> assets (lego_set_id));
joinable!(portfolios -> users (user_id));

allow_tables_to_appear_in_same_query!(
    assets,
    followers,
    portfolios,
    users,
);
