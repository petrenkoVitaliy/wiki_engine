use rocket::serde::Serialize;

#[derive(Clone, Serialize)]
pub struct LanguageAggregation {
    pub id: i32,
    pub code: String,
}
