use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::schemars::JsonSchema;
use std::collections::HashMap;

use super::diff_handler::DiffHandler;
use super::error::FmtError;

use super::repository::entity::version_content::{ContentType, VersionContent};

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct VersionContentAggregation {
    pub id: i32,
    pub content: String,
}

impl VersionContentAggregation {
    pub fn from_model(
        version_content: VersionContent,
        contents_map: Option<&HashMap<i32, String>>,
    ) -> Self {
        return VersionContentAggregation {
            id: version_content.id,

            content: Self::get_content(&version_content, contents_map),
        };
    }

    fn get_content(
        version_content: &VersionContent,
        contents_map: Option<&HashMap<i32, String>>,
    ) -> String {
        match version_content.content_type {
            ContentType::Diff => {
                if let Some(contents_map) = contents_map {
                    if let Some(content) = contents_map.get(&version_content.id) {
                        return String::from(content);
                    }
                }

                panic!(
                    "{}",
                    &FmtError::FailedToProcess("version_content_diff").fmt()
                );
            }
            ContentType::Full => DiffHandler::get_string_from_bytes(&version_content.content),
        }
    }
}
