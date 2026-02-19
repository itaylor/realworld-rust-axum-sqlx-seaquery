use sea_query::Value;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(transparent)]
#[schema(value_type = i64)]
pub struct CommentId(i64);

impl CommentId {
    pub fn value(&self) -> i64 {
        self.0
    }
}

impl From<i64> for CommentId {
    fn from(id: i64) -> Self {
        CommentId(id)
    }
}

impl From<CommentId> for i64 {
    fn from(id: CommentId) -> i64 {
        id.0
    }
}

impl Display for CommentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CommentId> for Value {
    fn from(id: CommentId) -> Self {
        Value::BigInt(Some(id.0))
    }
}
