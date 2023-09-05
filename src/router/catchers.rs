use rocket::{catch, catchers, Request};

#[catch(500)]
fn server_error() -> String {
    format!("Sorry, we messed up.")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

pub fn catchers() -> Vec<rocket::Catcher> {
    catchers![not_found, server_error,]
}
