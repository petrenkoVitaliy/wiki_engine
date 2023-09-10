use diesel::prelude::*;

use super::connection::PgConnection;
use super::error::FmtError;

use super::db_schema;
use super::model;

use super::dtm::auth::dto::{
    UserAccountCreateDto, UserOtpCreateDto, UserPasswordCreateDto, UserPatchDto,
};

pub struct AuthRepository;

impl AuthRepository {
    pub async fn get_one_user(connection: &PgConnection, id: i32) -> Option<model::UserAccount> {
        connection
            .run(move |connection| {
                db_schema::user_account::table
                    .filter(db_schema::user_account::id.eq(id))
                    .first(connection)
                    .optional()
            })
            .await
            .expect(&FmtError::FailedToFetch("user_account").fmt())
    }

    pub async fn get_one_user_with_password(
        connection: &PgConnection,
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
            .expect(&FmtError::FailedToFetch("user_account__user_password").fmt())
    }

    pub async fn get_one_user_with_otp(
        connection: &PgConnection,
        email: String,
    ) -> Option<(model::UserAccount, model::UserOtp)> {
        connection
            .run(|connection| {
                db_schema::user_account::table
                    .filter(db_schema::user_account::email.eq(email))
                    .inner_join(db_schema::user_otp::table)
                    .first(connection)
                    .optional()
            })
            .await
            .expect(&FmtError::FailedToFetch("user_account__user_password").fmt())
    }

    pub fn insert_user_account_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: UserAccountCreateDto,
    ) -> Result<model::UserAccount, diesel::result::Error> {
        diesel::insert_into(db_schema::user_account::table)
            .values(model::UserAccountInsertable {
                id: None,
                active: false,
                blocked: false,

                email: creation_dto.email,
                name: creation_dto.name,
                role_id: creation_dto.role_id,

                updated_at: None,
                created_at: None,
                updated_by: None,
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

    pub fn insert_user_otp_raw(
        connection: &mut diesel::PgConnection,
        creation_dto: UserOtpCreateDto,
    ) -> Result<model::UserOtp, diesel::result::Error> {
        diesel::insert_into(db_schema::user_otp::table)
            .values(model::UserOtpInsertable {
                id: None,

                user_id: creation_dto.user_id,
                otp: creation_dto.otp,
                created_at: None,
            })
            .get_result::<model::UserOtp>(connection)
    }

    pub fn delete_user_otp_raw(
        connection: &mut diesel::PgConnection,
        id: i32,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(db_schema::user_otp::table)
            .filter(db_schema::user_otp::id.eq(id))
            .execute(connection)
    }

    pub fn patch_raw(
        connection: &mut diesel::PgConnection,
        patch_dto: UserPatchDto,
    ) -> Result<model::UserAccount, diesel::result::Error> {
        diesel::update(db_schema::user_account::table)
            .filter(db_schema::user_account::id.eq(patch_dto.user_id))
            .set(model::UserAccountPatch {
                id: None,
                active: patch_dto.active,

                updated_by: None,
                blocked: None,
                email: None,
                name: None,
                role_id: None,
                created_at: None,
                updated_at: None,
            })
            .get_result::<model::UserAccount>(connection)
    }

    pub async fn patch(connection: &PgConnection, patch_dto: UserPatchDto) -> usize {
        connection
            .run(move |connection| {
                diesel::update(db_schema::user_account::table)
                    .filter(db_schema::user_account::id.eq(patch_dto.user_id))
                    .set(model::UserAccountPatch {
                        id: None,
                        blocked: patch_dto.blocked,
                        updated_by: patch_dto.updated_by,

                        active: None,
                        email: None,
                        name: None,
                        role_id: None,
                        created_at: None,
                        updated_at: None,
                    })
                    .execute(connection)
            })
            .await
            .expect(&FmtError::FailedToUpdate("user_account").fmt())
    }
}
