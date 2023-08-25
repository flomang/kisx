table! {
    article_tags (article_id, tag_name) {
        article_id -> Uuid,
        tag_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    articles (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        article_id -> Uuid,
        user_id -> Uuid,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    currencies (id) {
        id -> Uuid,
        name -> Text,
        symbol -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    favorite_articles (user_id, article_id) {
        user_id -> Uuid,
        article_id -> Uuid,
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
    lot_images (id) {
        id -> Uuid,
        lot_id -> Uuid,
        image_url -> Text,
        is_thumbnail -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    lot_statuses (description) {
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    lots (id) {
        id -> Uuid,
        user_id -> Uuid,
        category -> Text,
        condition -> Text,
        title -> Text,
        external_id -> Nullable<Text>,
        description -> Text,
        meta_data -> Nullable<Jsonb>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        status -> Text,
    }
}

table! {
    prices (recorded_at, external_id, source, currency_symbol) {
        external_id -> Text,
        source -> Text,
        currency_symbol -> Text,
        amount -> Numeric,
        recorded_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        email_verified -> Bool,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(article_tags -> articles (article_id));
joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));
joinable!(lot_images -> lots (lot_id));
joinable!(lots -> lot_statuses (status));
joinable!(lots -> users (user_id));

allow_tables_to_appear_in_same_query!(
    article_tags,
    articles,
    comments,
    currencies,
    favorite_articles,
    followers,
    lot_images,
    lot_statuses,
    lots,
    prices,
    users,
);
