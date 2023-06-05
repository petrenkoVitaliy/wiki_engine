pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "content_type"))]
    pub struct ContentType;
}

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
        content_id -> Int4,
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

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ContentType;

    version_content (id) {
        id -> Int4,
        content -> Bytea,
        content_type -> ContentType,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::joinable!(article_language -> article (article_id));
diesel::joinable!(article_language -> language (language_id));
diesel::joinable!(article_version -> article_language (article_language_id));
diesel::joinable!(article_version -> version_content (content_id));

diesel::allow_tables_to_appear_in_same_query!(article, article_language, article_version, language,);
