use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

use super::dtm_common::TokenDto;

use super::repository::entity::auth::UserAccount;

use super::user_account::UserAccountAggregation;

#[derive(Serialize, JsonSchema, Debug)]
pub struct UserAccountAuthAggregation {
    pub user: UserAccountAggregation,
    pub token: TokenDto,
}

impl UserAccountAuthAggregation {
    pub fn from_model(user_account: UserAccount, token: TokenDto) -> Self {
        Self {
            token,
            user: UserAccountAggregation::from_model(user_account),
        }
    }
}
