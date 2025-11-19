use crate::model::values::email::Email;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub user: LoginUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: Email,
    pub password: String,
}
