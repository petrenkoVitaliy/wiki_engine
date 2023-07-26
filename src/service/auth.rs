use diesel::Connection;

use super::error::{error_wrapper::ErrorWrapper, formatted_error::FmtError};
use super::hasher::Hasher;
use super::jwt_handler::JwtHandler;

use super::aggregation::user_account::UserAccountAggregation;

use super::schema::auth::{
    UserAccountCreateDto, UserCreateRelationsDto, UserLoginBody, UserPasswordCreateDto,
    UserPatchDto, UserSignupBody,
};
use super::schema::jwt::TokenResponse;

use super::repository::connection;
use super::repository::entity::auth::{AuthRepository, UserAccount, UserPassword};

pub struct AuthService;

impl AuthService {
    pub async fn get_aggregation(
        connection: &connection::PgConnection,
        user_id: i32,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        let user_account = match AuthRepository::get_one_user(connection, user_id).await {
            None => return FmtError::NotFound("user_account").error(),
            Some(user_account) => user_account,
        };

        Ok(UserAccountAggregation::from_model(user_account))
    }

    pub async fn login(
        connection: &connection::PgConnection,
        user_signup_body: UserLoginBody,
    ) -> Result<TokenResponse, ErrorWrapper> {
        let (user_password, user_account) =
            match AuthRepository::get_one_user_with_password(connection, user_signup_body.email)
                .await
            {
                Some((user_password, user_account)) => (user_password, user_account),
                None => return FmtError::Unauthorized("invalid credentials").error(),
            };

        if !user_account.active {
            return FmtError::PermissionDenied("not enough rights").error();
        }

        match Hasher::verify_encoded(user_signup_body.password, user_password.password) {
            Ok(is_correct) => match is_correct {
                false => return FmtError::Unauthorized("invalid credentials").error(),
                _ => (),
            },
            Err(e) => return Err(e),
        };

        match JwtHandler::encode_jwt(user_account.id) {
            Ok(jwt_string) => Ok(TokenResponse { token: jwt_string }),
            Err(e) => return Err(e),
        }
    }

    pub async fn create_user(
        connection: &connection::PgConnection,
        user_signup_body: UserSignupBody,
    ) -> Result<UserAccountAggregation, ErrorWrapper> {
        // TODO if exists??
        let password_hash = match Hasher::hash_password(user_signup_body.password) {
            Ok(password_hash) => password_hash,
            Err(e) => return Err(e),
        };

        let user_account = match Self::create_relations_transaction(
            connection,
            UserCreateRelationsDto {
                password_hash,
                email: user_signup_body.email,
                name: user_signup_body.name,
            },
        )
        .await
        {
            Ok((user_account, _)) => user_account,
            Err(_) => return FmtError::FailedToInsert("user_account").error(),
        };

        Ok(UserAccountAggregation::from_model(user_account))
    }

    pub async fn patch(
        connection: &connection::PgConnection,
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
        connection: &connection::PgConnection,
        creation_dto: UserCreateRelationsDto,
    ) -> Result<(UserAccount, UserPassword), diesel::result::Error> {
        connection
            .run(move |connection| {
                return connection
                    .transaction::<(UserAccount, UserPassword), diesel::result::Error, _>(
                        |transaction_connection| {
                            Ok(Self::create_relations(transaction_connection, creation_dto))
                        },
                    );
            })
            .await
    }

    fn create_relations(
        connection: &mut diesel::PgConnection,
        creation_dto: UserCreateRelationsDto,
    ) -> (UserAccount, UserPassword) {
        let user_account = AuthRepository::insert_user_account_raw(
            connection,
            UserAccountCreateDto {
                email: creation_dto.email,
                name: creation_dto.name,
            },
        )
        .expect(&FmtError::FailedToInsert("user_account").fmt());

        let user_password = AuthRepository::insert_user_password_raw(
            connection,
            UserPasswordCreateDto {
                password_hash: creation_dto.password_hash,
                user_id: user_account.id,
            },
        )
        .expect(&FmtError::FailedToInsert("user_password").fmt());

        (user_account, user_password)
    }
}
