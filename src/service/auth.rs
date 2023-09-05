use diesel::Connection;

use super::dtm_common::{TokenDto, UserRoleId};
use super::error::{ErrorWrapper, FmtError};
use super::hasher::Hasher;
use super::jwt_handler::JwtHandler;

use super::dtm::auth::dto::{
    UserAccountCreateDto, UserCreateRelationsDto, UserLoginDto, UserPasswordCreateDto,
    UserPatchDto, UserSignupDto,
};

use super::aggregation::user_account::UserAccountAggregation;
use super::aggregation::user_account_auth::UserAccountAuthAggregation;

use super::repository::{
    entity::auth::{AuthRepository, UserAccount, UserPassword},
    PgConnection,
};

pub struct AuthService;

impl AuthService {
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

        if !user_account.active {
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

    pub async fn create_user(
        connection: &PgConnection,
        user_signup_dto: UserSignupDto,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        let password_hash = match Hasher::hash_password(user_signup_dto.password) {
            Ok(password_hash) => password_hash,
            Err(e) => return Err(e),
        };

        let (user_account, _) = Self::create_relations_transaction(
            connection,
            UserCreateRelationsDto {
                password_hash,
                email: user_signup_dto.email,
                name: user_signup_dto.name,
                role_id: UserRoleId::Common as i32,
            },
        )
        .await?;

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

        let (user_account, _) = Self::create_relations_transaction(
            connection,
            UserCreateRelationsDto {
                role_id,
                password_hash,
                email: user_signup_dto.email,
                name: user_signup_dto.name,
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

    async fn create_relations_transaction(
        connection: &PgConnection,
        creation_dto: UserCreateRelationsDto,
    ) -> Result<(UserAccount, UserPassword), ErrorWrapper> {
        connection
            .run(move |connection| {
                return connection.transaction::<(UserAccount, UserPassword), ErrorWrapper, _>(
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
    ) -> Result<(UserAccount, UserPassword), ErrorWrapper> {
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

        Ok((user_account, user_password))
    }
}
