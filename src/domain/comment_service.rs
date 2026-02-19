use crate::app_error::AppError;
use crate::domain::commands::add_comment_command::AddCommentCommand;
use crate::model::persistence::comment_view::CommentView;
use crate::model::values::article_id::ArticleId;
use crate::model::values::comment_id::CommentId;
use crate::model::values::user_id::UserId;
use crate::persistence::comment_repository::CommentRepository;
use anyhow::Result;
use tracing::info;

#[derive(Clone)]
pub struct CommentService {
    comment_repo: CommentRepository,
}

impl CommentService {
    pub fn new(comment_repo: CommentRepository) -> Self {
        CommentService { comment_repo }
    }

    pub async fn delete_comment(
        &self,
        comment_id: CommentId,
        user_id: UserId,
    ) -> Result<(), AppError> {
        match self.comment_repo.get_comment_author(comment_id).await? {
            None => {
                info!("Comment {} not found", comment_id);
                Err(AppError::ResourceNotFound("comment"))
            }
            Some(author_id) if author_id != user_id => Err(AppError::ResourceForbidden("comment")),
            Some(_) => self.comment_repo.delete_comment(comment_id).await,
        }
    }

    pub async fn add_comment(
        &self,
        command: AddCommentCommand,
        user_id: UserId,
    ) -> Result<CommentView, AppError> {
        let params = command.to_insert_params();
        let comment = self.comment_repo.insert_comment(params).await?;

        let comment = self
            .comment_repo
            .get_comment(comment.id, Some(user_id))
            .await?;

        Ok(comment)
    }

    pub async fn get_comments(
        &self,
        article_id: ArticleId,
        user_id: Option<UserId>,
    ) -> Result<Vec<CommentView>, AppError> {
        self.comment_repo.get_comments(article_id, user_id).await
    }
}
