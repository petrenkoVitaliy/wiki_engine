use std::collections::HashMap;

use super::repository::module::version_content::model::VersionContent;

use super::schema::version_content::{ContentType, VersionContentAggregation};

use super::diff_handler::diff_handler::DiffHandler;

pub struct VersionContentMapper {}

impl VersionContentMapper {
    pub fn map_to_aggregations(
        version_content: Vec<VersionContent>,
    ) -> Vec<VersionContentAggregation> {
        version_content
            .into_iter()
            .map(move |version_content| {
                return VersionContentAggregation {
                    id: version_content.id,

                    content: Self::get_content(&version_content, None),
                };
            })
            .collect()
    }

    pub fn map_to_aggregation(
        version_content: VersionContent,
        contents_map: Option<&HashMap<i32, String>>,
    ) -> VersionContentAggregation {
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

                String::from("_patch_")
            }
            ContentType::Full => DiffHandler::get_string_from_bytes(&version_content.content),
        }
    }
}
