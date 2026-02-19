use crate::model::values::bio::Bio;
use crate::model::values::email::Email;
use crate::model::values::image::Image;
use crate::model::values::password_hash::PasswordHash;
use crate::model::values::user_id::UserId;
use crate::model::values::username::Username;
use crate::persistence::schema::Users;
use sea_query::Value;

pub struct UpdateUserParams {
    pub(crate) user_id: UserId,
    pub(crate) email: Option<Email>,
    pub(crate) username: Option<Username>,
    pub(crate) password_hash: Option<PasswordHash>,
    pub(crate) bio: Option<Bio>,
    pub(crate) image: Option<Image>,
}

pub struct ColumnUpdate {
    pub column: Users,
    pub value: Value,
}

impl UpdateUserParams {
    pub fn as_list(&self) -> Vec<ColumnUpdate> {
        let mut fields = Vec::new();

        if let Some(email) = &self.email {
            fields.push(ColumnUpdate {
                column: Users::Email,
                value: Value::String(Some(Box::new(email.value().to_string()))),
            });
        }
        if let Some(username) = &self.username {
            fields.push(ColumnUpdate {
                column: Users::Username,
                value: Value::String(Some(Box::new(username.value().to_string()))),
            });
        }
        if let Some(password_hash) = &self.password_hash {
            fields.push(ColumnUpdate {
                column: Users::PasswordHash,
                value: Value::String(Some(Box::new(password_hash.value().to_string()))),
            });
        }
        if let Some(bio) = &self.bio {
            // Empty bio normalizes to NULL
            fields.push(ColumnUpdate {
                column: Users::Bio,
                value: if bio.is_empty() {
                    Value::String(None)
                } else {
                    Value::String(Some(Box::new(bio.value().to_string())))
                },
            });
        }
        if let Some(image) = &self.image {
            // Empty image normalizes to NULL
            fields.push(ColumnUpdate {
                column: Users::Image,
                value: if image.is_empty() {
                    Value::String(None)
                } else {
                    Value::String(Some(Box::new(image.value().to_string())))
                },
            });
        }

        fields
    }
}
