use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

use super::authorization::ArticlePermission;
use super::dtm_common::TokenDto;

use super::repository::entity::auth::UserAccount;

use super::user_account::{UserAccountAggregation, UserAccountPartialAggregation};

#[derive(Serialize, JsonSchema)]
pub struct UserAccountAuthAggregation {
    pub user: UserAccountPartialAggregation,
    pub token: TokenDto,
}

impl UserAccountAuthAggregation {
    pub fn from_model(user_account: UserAccount, token: TokenDto) -> Self {
        Self {
            token,
            user: UserAccountPartialAggregation::from_model(user_account),
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct UserAccountPermissionsAggregation {
    pub user: UserAccountPartialAggregation,
    pub permissions: Vec<ArticlePermission>,
}

impl UserAccountPermissionsAggregation {
    pub fn from_aggregation(
        user_account_aggregation: UserAccountAggregation,
        permissions: Vec<ArticlePermission>,
    ) -> Self {
        Self {
            permissions,
            user: UserAccountPartialAggregation::from_aggregation(user_account_aggregation),
        }
    }
}
