use super::error::formatted_error::FmtError;

use super::repository::module::version_content::model::VersionContent;

use super::schema::version_content::{ContentType, VersionContentAggregation};

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

                    content: Self::get_content(&version_content),

                    updated_at: version_content.updated_at,
                    created_at: version_content.created_at,
                };
            })
            .collect()
    }

    pub fn map_to_aggregation(version_content: VersionContent) -> VersionContentAggregation {
        return VersionContentAggregation {
            id: version_content.id,

            content: Self::get_content(&version_content),

            updated_at: version_content.updated_at,
            created_at: version_content.created_at,
        };
    }

    fn get_content(version_content: &VersionContent) -> String {
        let content = (&version_content.content).to_vec();

        match version_content.content_type {
            ContentType::Diff => String::from("patch"),
            ContentType::Full => String::from_utf8(content)
                .expect(FmtError::FailedToProcess("content").fmt().as_str()),
        }
    }
}
