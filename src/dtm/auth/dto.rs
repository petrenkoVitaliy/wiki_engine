use super::repository::entity::auth::OTPType;

pub struct UserAccountCreateDto {
    pub email: String,
    pub name: String,
    pub role_id: i32,
}

pub struct UserPasswordCreateDto {
    pub user_id: i32,
    pub password_hash: String,
}

pub struct UserOtpCreateDto {
    pub user_id: i32,
    pub otp: String,
    pub otp_type: OTPType,
}

pub struct UserPatchDto {
    pub user_id: i32,
    pub updated_by: Option<i32>,
    pub active: Option<bool>,
    pub blocked: Option<bool>,
}

pub struct UserCreateRelationsDto {
    pub email: String,
    pub name: String,
    pub password_hash: String,
    pub role_id: i32,
    pub otp: Option<String>,
}

#[derive(Debug)]
pub struct UserResetOTPsDto {
    pub user_id: i32,
    pub existing_otp_ids: Vec<i32>,
    pub otp: String,
}

pub struct UserSignupDto {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserResetDto {
    pub email: String,
}

pub struct UserConfirmDto {
    pub email: String,
    pub otp: String,
}

pub struct UserConfirmPasswordResetDto {
    pub email: String,
    pub otp: String,
    pub password: String,
}

pub struct UserLoginDto {
    pub email: String,
    pub password: String,
}
