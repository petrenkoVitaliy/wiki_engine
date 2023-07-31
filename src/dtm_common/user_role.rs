#[derive(PartialEq, Clone, Debug)]
pub enum UserRoleId {
    Common = 1,
    Moderator = 2,
    Admin = 3,
}

impl UserRoleId {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            1 => Some(UserRoleId::Common),
            2 => Some(UserRoleId::Moderator),
            3 => Some(UserRoleId::Admin),
            _ => None,
        }
    }
}
