use rocket::http::Header;

pub struct RequestHandler;
impl RequestHandler {
    pub fn get_auth_header<'s>(jwt_token: String) -> Header<'s> {
        Header::new("Authorization", format!("Bearer {}", jwt_token))
    }
}
