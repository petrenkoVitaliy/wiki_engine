pub struct UserAccountCreateDto {
    pub email: String,
    pub name: String,
    pub role_id: i32,
}

pub struct UserPasswordCreateDto {
    pub user_id: i32,
    pub password_hash: String,
}

pub struct UserPatchDto {
    pub active: bool,
    pub user_id: i32,
    pub updated_by: i32,
}

pub struct UserCreateRelationsDto {
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub role_id: i32,
}

pub struct UserSignupDto {
    pub email: String,
    pub name: String,
    pub password: String,
}

pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}
