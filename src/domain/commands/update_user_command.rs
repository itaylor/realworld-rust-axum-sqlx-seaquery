use uuid::Uuid;
use crate::http::dto::user::UpdateUserRequest;
use crate::model::values::email::Email;
use crate::model::values::username::Username;

#[derive(Debug, Clone)]
pub struct UpdateUserCommand {
  pub user_id: Uuid,
  pub email: Option<Email>,
  pub username: Option<Username>,
  pub password: Option<String>,
  pub bio: Option<String>,
  pub image: Option<String>,
}

impl UpdateUserCommand {
  pub(crate) fn new(req: UpdateUserRequest, user_id: Uuid) -> Self {
      UpdateUserCommand {
          user_id,
          email: req.user.email,
          username: req.user.username,
          password: req.user.password,
          bio: req.user.bio,
          image: req.user.image,
      }
  }
}
