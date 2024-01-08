pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "content_type"))]
    pub struct ContentType;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "article_type"))]
    pub struct ArticleType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ArticleType;

    article (id) {
        id -> Int4,
        enabled -> Bool,
        archived -> Bool,
        article_type -> ArticleType,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_by -> Nullable<Int4>,
        created_by -> Int4,
    }
}

diesel::table! {
    article_language (id) {
        id -> Int4,
        name -> Varchar,
        name_key -> Varchar,
        enabled -> Bool,
        archived -> Bool,
        article_id -> Int4,
        language_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_by -> Nullable<Int4>,
        created_by -> Int4,
    }
}

diesel::table! {
    article_version (id) {
        id -> Int4,
        version -> Int4,
        content_id -> Int4,
        enabled -> Bool,
        name -> Varchar,
        article_language_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_by -> Nullable<Int4>,
        created_by -> Int4,
    }
}

diesel::table! {
    user_password (id) {
        id -> Int4,
        password -> Varchar,
        user_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_otp (id) {
        id -> Int4,
        otp -> Varchar,
        user_id -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    user_account (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        active -> Bool,
        blocked -> Bool,
        role_id -> Int4,
        updated_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_by -> Nullable<Int4>,
    }
}

diesel::table! {
    user_role (id) {
        id -> Int4,
        role -> Varchar,
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
        content_length -> Int4,
    }
}

diesel::joinable!(article_language -> article (article_id));
diesel::joinable!(article_language -> language (language_id));

diesel::joinable!(article_version -> article_language (article_language_id));
diesel::joinable!(article_version -> version_content (content_id));
diesel::joinable!(article_version -> user_account (created_by));

diesel::joinable!(user_otp -> user_account (user_id));

diesel::joinable!(user_password -> user_account (user_id));

diesel::joinable!(user_account -> user_role (role_id));

diesel::allow_tables_to_appear_in_same_query!(
    article,
    article_language,
    article_version,
    version_content,
    language,
    user_password,
    user_otp,
    user_account,
    user_role,
);
