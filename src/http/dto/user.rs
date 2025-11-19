use crate::model::values::email::Email;
use crate::model::values::username::Username;
use serde::{Deserialize, Serialize};
use crate::model::persistence::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub user: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub email: Email,
    pub token: String,
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl UserData {
    pub(crate) fn new(user: User, token: String) -> Self {
        UserData {
            email: user.email,
            token,
            username: user.username,
            bio: user.bio,
            image: user.image,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub user: UpdateUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<Username>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
}
