use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;

use super::repository::entity::auth::UserAccount;

#[derive(Serialize, JsonSchema, Debug, Deserialize)]
pub struct UserAccountAggregation {
    pub id: i32,

    pub email: String,
    pub name: String,
    pub active: bool,

    pub role_id: i32,

    pub updated_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

impl UserAccountAggregation {
    pub fn from_model(user_account: UserAccount) -> Self {
        Self {
            id: user_account.id,
            email: user_account.email,
            active: user_account.active,
            name: user_account.name,
            role_id: user_account.role_id,

            updated_at: user_account.updated_at,
            created_at: user_account.created_at,
        }
    }
}
