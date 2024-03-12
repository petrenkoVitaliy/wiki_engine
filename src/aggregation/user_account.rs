use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::repository::entity::auth::UserAccount;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct UserAccountPartialAggregation {
    pub email: String,
    pub name: String,

    pub role_id: i32,
}

impl UserAccountPartialAggregation {
    pub fn from_model(user_account: UserAccount) -> Self {
        Self {
            email: user_account.email,
            name: user_account.name,
            role_id: user_account.role_id,
        }
    }

    pub fn from_aggregation(user_account_aggregation: UserAccountAggregation) -> Self {
        Self {
            email: user_account_aggregation.email,
            name: user_account_aggregation.name,
            role_id: user_account_aggregation.role_id,
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct UserAccountAggregation {
    pub id: i32,

    pub email: String,
    pub name: String,
    pub active: bool,
    pub blocked: bool,

    pub role_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,

    pub updated_by: Option<i32>,
}

impl UserAccountAggregation {
    pub fn from_model(user_account: UserAccount) -> Self {
        Self {
            id: user_account.id,
            email: user_account.email,
            active: user_account.active,
            blocked: user_account.blocked,
            name: user_account.name,
            role_id: user_account.role_id,

            updated_at: user_account.updated_at,
            created_at: user_account.created_at,

            updated_by: user_account.updated_by,
        }
    }
}
