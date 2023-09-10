use diesel::Connection;
use uuid::Uuid;

use super::authorization::PermissionsHandler;
use super::dtm_common::{QueryOptions, TokenDto, UserRoleId};
use super::emailer::Emailer;
use super::error::{ErrorWrapper, FmtError};
use super::hasher::Hasher;
use super::jwt_handler::JwtHandler;

use super::dtm::auth::dto::{
    UserAccountCreateDto, UserConfirmDto, UserCreateRelationsDto, UserLoginDto, UserOtpCreateDto,
    UserPasswordCreateDto, UserPatchDto, UserSignupDto,
};

use super::aggregation::user_account::UserAccountAggregation;
use super::aggregation::user_account_auth::{
    UserAccountAuthAggregation, UserAccountPermissionsAggregation,
};

use super::repository::{
    entity::{
        article::ArticleRepository,
        auth::{AuthRepository, UserAccount, UserOtp, UserPassword},
    },
    PgConnection,
};

pub struct AuthService;

impl AuthService {
    pub async fn get_user_with_permissions(
        connection: &PgConnection,
        user_account: UserAccountAggregation,
        article_id: Option<i32>,
    ) -> UserAccountPermissionsAggregation {
        let permissions = match article_id {
            None => vec![],
            Some(article_id) => {
                match ArticleRepository::get_one(
                    connection,
                    article_id,
                    &QueryOptions { is_actual: true },
                )
                .await
                {
                    None => vec![],
                    Some(article) => PermissionsHandler::get_permissions(&article, &user_account),
                }
            }
        };

        UserAccountPermissionsAggregation::from_aggregation(user_account, permissions)
    }

    pub async fn get_aggregation(
        connection: &PgConnection,
        user_id: i32,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        let user_account = match AuthRepository::get_one_user(connection, user_id).await {
            None => return FmtError::NotFound("user_account").error(),
            Some(user_account) => user_account,
        };

        Ok(UserAccountAggregation::from_model(user_account))
    }

    pub async fn login(
        connection: &PgConnection,
        user_signup_dto: UserLoginDto,
    ) -> Result<UserAccountAuthAggregation, ErrorWrapper> {
        let (user_password, user_account) =
            match AuthRepository::get_one_user_with_password(connection, user_signup_dto.email)
                .await
            {
                Some((user_password, user_account)) => (user_password, user_account),
                None => return FmtError::Unauthorized("invalid credentials").error(),
            };

        if !user_account.active || user_account.blocked {
            return FmtError::PermissionDenied("not enough rights").error();
        }

        match Hasher::verify_encoded(user_signup_dto.password, user_password.password) {
            Ok(is_correct) => match is_correct {
                false => return FmtError::Unauthorized("invalid credentials").error(),
                _ => (),
            },
            Err(e) => return Err(e),
        };

        let jwt_string = match JwtHandler::encode_jwt(user_account.id) {
            Ok(jwt_string) => jwt_string,
            Err(e) => return Err(e),
        };

        Ok(UserAccountAuthAggregation::from_model(
            user_account,
            TokenDto { token: jwt_string },
        ))
    }

    pub async fn confirm_user(
        connection: &PgConnection,
        user_confirm_dto: UserConfirmDto,
    ) -> Result<UserAccountAuthAggregation, ErrorWrapper> {
        let (user_account, user_otp) =
            match AuthRepository::get_one_user_with_otp(connection, user_confirm_dto.email).await {
                Some((user_password, user_account)) => (user_password, user_account),
                None => return FmtError::Unauthorized("invalid credentials1").error(),
            };

        if user_otp.otp != user_confirm_dto.otp {
            return FmtError::Unauthorized("invalid credentials").error();
        }

        let user_account =
            Self::confirm_user_transaction(connection, user_account.id, user_otp.id).await?;

        let jwt_string = match JwtHandler::encode_jwt(user_account.id) {
            Ok(jwt_string) => jwt_string,
            Err(e) => return Err(e),
        };

        Ok(UserAccountAuthAggregation::from_model(
            user_account,
            TokenDto { token: jwt_string },
        ))
    }

    pub async fn create_user(
        connection: &PgConnection,
        user_signup_dto: UserSignupDto,
        redirect_to: Option<String>,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        let password_hash = match Hasher::hash_password(user_signup_dto.password) {
            Ok(password_hash) => password_hash,
            Err(e) => return Err(e),
        };

        let otp = Uuid::new_v4().to_string();

        let (user_account, _, _) = Self::create_relations_transaction(
            connection,
            UserCreateRelationsDto {
                password_hash,
                email: user_signup_dto.email,
                name: user_signup_dto.name,
                role_id: UserRoleId::Common as i32,
                otp: Some(otp.clone()),
            },
        )
        .await?;

        Emailer::send_confirmation_email(&otp, &user_account.email, &redirect_to).await?;

        Ok(UserAccountAggregation::from_model(user_account))
    }

    pub async fn create_user_with_role(
        connection: &PgConnection,
        user_signup_dto: UserSignupDto,
        role_id: i32,
    ) -> Result<TokenDto, ErrorWrapper> {
        let password_hash = match Hasher::hash_password(user_signup_dto.password) {
            Ok(password_hash) => password_hash,
            Err(e) => return Err(e),
        };

        let (user_account, _, _) = Self::create_relations_transaction(
            connection,
            UserCreateRelationsDto {
                role_id,
                password_hash,
                email: user_signup_dto.email,
                name: user_signup_dto.name,
                otp: None,
            },
        )
        .await?;

        match JwtHandler::encode_jwt(user_account.id) {
            Ok(jwt_string) => Ok(TokenDto { token: jwt_string }),
            Err(e) => return Err(e),
        }
    }

    pub async fn patch(
        connection: &PgConnection,
        patch_dto: UserPatchDto,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        let user_id = patch_dto.user_id;
        let updated_count = AuthRepository::patch(connection, patch_dto).await;

        if updated_count == 0 {
            return FmtError::NotFound("user_account").error();
        }

        Self::get_aggregation(connection, user_id).await
    }

    async fn confirm_user_transaction(
        connection: &PgConnection,
        user_id: i32,
        otp_id: i32,
    ) -> Result<UserAccount, ErrorWrapper> {
        connection
            .run(move |connection| {
                return connection.transaction::<UserAccount, ErrorWrapper, _>(
                    |transaction_connection| {
                        Self::confirm_user_relations(transaction_connection, user_id, otp_id)
                    },
                );
            })
            .await
    }

    fn confirm_user_relations(
        connection: &mut diesel::PgConnection,
        user_id: i32,
        otp_id: i32,
    ) -> Result<UserAccount, ErrorWrapper> {
        let user_account = AuthRepository::patch_raw(
            connection,
            UserPatchDto {
                user_id,
                active: Some(true),

                updated_by: None,
                blocked: None,
            },
        )
        .expect(&FmtError::FailedToUpdate("user_account").fmt());

        AuthRepository::delete_user_otp_raw(connection, otp_id)
            .expect(&FmtError::FailedToUpdate("user_otp").fmt());

        Ok(user_account)
    }

    async fn create_relations_transaction(
        connection: &PgConnection,
        creation_dto: UserCreateRelationsDto,
    ) -> Result<(UserAccount, UserPassword, Option<UserOtp>), ErrorWrapper> {
        connection
            .run(move |connection| {
                return connection
                    .transaction::<(UserAccount, UserPassword, Option<UserOtp>), ErrorWrapper, _>(
                        |transaction_connection| {
                            Self::create_relations(transaction_connection, creation_dto)
                        },
                    );
            })
            .await
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: UserCreateRelationsDto,
    ) -> Result<(UserAccount, UserPassword, Option<UserOtp>), ErrorWrapper> {
        let user_account = match AuthRepository::insert_user_account_raw(
            connection,
            UserAccountCreateDto {
                email: creation_dto.email,
                name: creation_dto.name,
                role_id: creation_dto.role_id,
            },
        ) {
            Ok(user_account) => user_account,
            Err(e) => {
                return Err(ErrorWrapper::from_duplicated_key(
                    e,
                    FmtError::FailedToInsert("user_account").error_wrapper(),
                ));
            }
        };

        let user_password = AuthRepository::insert_user_password_raw(
            connection,
            UserPasswordCreateDto {
                password_hash: creation_dto.password_hash,
                user_id: user_account.id,
            },
        )
        .expect(&FmtError::FailedToInsert("user_password").fmt());

        let user_otp = match creation_dto.otp {
            None => None,
            Some(otp) => {
                let otp = AuthRepository::insert_user_otp_raw(
                    connection,
                    UserOtpCreateDto {
                        otp,
                        user_id: user_account.id,
                    },
                )
                .expect(&FmtError::FailedToInsert("user_otp").fmt());

                Some(otp)
            }
        };

        Ok((user_account, user_password, user_otp))
    }
}
