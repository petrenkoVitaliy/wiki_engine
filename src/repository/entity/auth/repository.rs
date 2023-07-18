use diesel::prelude::*;

use super::connection;
use super::db_schema;
use super::model;

use super::error::formatted_error::FmtError;

use super::schema::auth::{UserAccountCreateDto, UserPasswordCreateDto};

pub struct AuthRepository;

impl AuthRepository {
    pub async fn get_one_user_with_password(
        connection: &connection::PgConnection,
        email: String,
    ) -> Option<(model::UserPassword, model::UserAccount)> {
        connection
            .run(|connection| {
                db_schema::user_password::table
                    .inner_join(db_schema::user_account::table)
                    .filter(db_schema::user_account::email.eq(email))
                    .first(connection)
                    .optional()
            })
            .await
            .expect(
                FmtError::FailedToFetch("user_account__user_password")
                    .fmt()
                    .as_str(),
            )
    }

    pub fn insert_user_account_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: UserAccountCreateDto,
    ) -> Result<model::UserAccount, diesel::result::Error> {
        diesel::insert_into(db_schema::user_account::table)
            .values(model::UserAccountInsertable {
                id: None,

                email: creation_dto.email,
                name: creation_dto.name,

                role_id: 1, // TODO default role

                updated_at: None,
                created_at: None,
            })
            .get_result::<model::UserAccount>(connection)
    }

    pub fn insert_user_password_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: UserPasswordCreateDto,
    ) -> Result<model::UserPassword, diesel::result::Error> {
        diesel::insert_into(db_schema::user_password::table)
            .values(model::UserPasswordInsertable {
                id: None,

                user_id: creation_dto.user_id,
                password: creation_dto.password_hash,
                updated_at: None,
                created_at: None,
            })
            .get_result::<model::UserPassword>(connection)
    }
}