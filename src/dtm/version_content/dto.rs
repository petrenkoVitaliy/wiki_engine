use super::repository::entity::version_content::ContentType;

pub struct VersionContentDto {
    pub content: Vec<u8>,
    pub content_type: ContentType,
}
