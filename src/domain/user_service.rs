use crate::app_error::AppError;
use crate::domain::register_command::RegisterCommand;
use crate::domain::user_repository::UserRepository;
use crate::model::persistence::user::User;
use anyhow::Result;
use crate::utils::hasher::Hasher;

#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
    hasher: Hasher,
}

impl UserService {
    pub fn new(user_repo: UserRepository, hasher: Hasher) -> Self {
        UserService { user_repo, hasher }
    }

    pub async fn register_user(&self, request: RegisterCommand) -> Result<User, AppError> {
        let password_hash = self.hasher.hash_password(&request.password)?;

        if self.user_repo.get_user_by_username(request.username.clone()).await?.is_some() {
            return Err(AppError::Conflict(format!(
                "Username '{}' is already taken",
                request.username
            )));
        } else if self.user_repo.get_user_by_email(request.email.clone()).await?.is_some() {
            return Err(AppError::Conflict(format!(
                "Email '{}' is already registered",
                request.email
            )));
        }

        let user = self
            .user_repo
            .insert_user(&request.email, &request.username, &password_hash)
            .await?;

        Ok(user)
    }
}
