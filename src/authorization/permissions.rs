use rocket::serde::Serialize;
use rocket_okapi::okapi::schemars::JsonSchema;

use super::repository::entity::article::{Article, ArticleType};

use super::aggregation::user_account::UserAccountAggregation;
use super::dtm_common::UserRoleId;

#[derive(Serialize, JsonSchema, Debug)]
pub enum ArticlePermission {
    Edit,
    Patch,
}

#[derive(Debug)]
pub struct PermissionsHandler;
impl PermissionsHandler {
    pub fn get_permissions(
        article: &Article,
        current_user: &UserAccountAggregation,
    ) -> Vec<ArticlePermission> {
        let user_role = UserRoleId::from_i32(current_user.role_id).unwrap_or(UserRoleId::Common);

        if user_role == UserRoleId::Admin || user_role == UserRoleId::Moderator {
            return vec![ArticlePermission::Edit, ArticlePermission::Patch];
        }

        if current_user.id == article.created_by {
            return vec![ArticlePermission::Edit, ArticlePermission::Patch];
        }

        if article.article_type == ArticleType::Public {
            return vec![ArticlePermission::Edit];
        }

        return vec![];
    }

    pub fn can_patch_article(article: &Article, current_user: &UserAccountAggregation) -> bool {
        Self::get_permissions(article, current_user)
            .into_iter()
            .find(|permission| matches!(*permission, ArticlePermission::Patch))
            .is_some()
    }

    pub fn can_create_article_version(
        article: &Article,
        current_user: &UserAccountAggregation,
    ) -> bool {
        Self::get_permissions(article, current_user)
            .into_iter()
            .find(|permission| matches!(*permission, ArticlePermission::Edit))
            .is_some()
    }
}
