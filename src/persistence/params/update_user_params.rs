use uuid::Uuid;
use crate::model::values::email::Email;
use crate::model::values::username::Username;

pub struct UpdateUserParams {
  pub(crate) user_id: Uuid,
  pub(crate) email: Option<Email>,
  pub(crate) username: Option<Username>,
  pub(crate) password_hash: Option<String>,
  pub(crate) bio: Option<String>,
  pub(crate) image: Option<String>,
}

impl UpdateUserParams {
  pub fn as_list(&self) -> Vec<(String, String)> {
    let mut fields = Vec::new();

    if let Some(email) = &self.email {
      fields.push(("email".to_string(), email.value().to_string()));
    }
    if let Some(username) = &self.username {
      fields.push(("username".to_string(), username.value().to_string()));
    }
    if let Some(password_hash) = &self.password_hash {
      fields.push(("password_hash".to_string(), password_hash.clone()));
    }
    if let Some(bio) = &self.bio {
      fields.push(("bio".to_string(), bio.clone()));
    }
    if let Some(image) = &self.image {
      fields.push(("image".to_string(), image.clone()));
    }

    fields
  }
}
