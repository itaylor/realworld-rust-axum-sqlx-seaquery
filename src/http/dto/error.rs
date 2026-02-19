use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub errors: HashMap<String, Vec<String>>,
}

impl ErrorResponse {
    pub fn field(field: impl Into<String>, message: impl Into<String>) -> Self {
        let mut errors = HashMap::new();
        errors.insert(field.into(), vec![message.into()]);
        Self { errors }
    }
}
