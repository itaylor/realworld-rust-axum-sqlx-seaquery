use crate::model::values::email::Email;
use crate::model::values::username::Username;
use sqlx::Row;
use sqlx::postgres::PgRow;
use uuid::Uuid;

pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub password_hash: String,
    pub username: Username,
    pub bio: Option<String>,
    pub image: Option<String>,
}

impl User {
    pub fn from_row(row: PgRow) -> Self {
        Self {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            bio: row.get("bio"),
            image: row.get("image"),
        }
    }
}
