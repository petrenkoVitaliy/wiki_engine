// @generated automatically by Diesel CLI.

diesel::table! {
    article (id) {
        id -> Int4,
        enabled -> Bool,
        archived -> Bool,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    article_language (id) {
        id -> Int4,
        name -> Varchar,
        enabled -> Bool,
        archived -> Bool,
        article_id -> Int4,
        language_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    article_version (id) {
        id -> Int4,
        version -> Int4,
        content -> Varchar,
        enabled -> Bool,
        article_language_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    language (id) {
        id -> Int4,
        code -> Varchar,
    }
}

diesel::joinable!(article_language -> article (article_id));
diesel::joinable!(article_language -> language (language_id));
diesel::joinable!(article_version -> article_language (article_language_id));

diesel::allow_tables_to_appear_in_same_query!(article, article_language, article_version, language,);
