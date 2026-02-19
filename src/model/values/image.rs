use sea_query::Value;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::Type;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Type, ToSchema)]
#[sqlx(transparent)]
#[serde(try_from = "String")]
#[schema(value_type = String, example = "https://example.com/avatar.jpg")]
pub struct Image(String);

impl Serialize for Image {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if self.0.is_empty() {
            serializer.serialize_none()
        } else {
            serializer.serialize_str(&self.0)
        }
    }
}

impl Image {
    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl TryFrom<String> for Image {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let trimmed = value.trim();

        if trimmed.is_empty() {
            return Ok(Image(String::new()));
        }

        if !trimmed.starts_with("http://") && !trimmed.starts_with("https://") {
            return Err("Image URL must start with http:// or https://".to_string());
        }

        if trimmed.len() > 2048 {
            return Err("Image URL cannot be longer than 2048 characters".to_string());
        }

        Ok(Image(trimmed.to_string()))
    }
}

impl TryFrom<&str> for Image {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.to_string().try_into()
    }
}

impl From<Image> for String {
    fn from(image: Image) -> String {
        image.0
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for Image {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Image> for Value {
    fn from(i: Image) -> Self {
        if i.0.is_empty() {
            Value::String(None)
        } else {
            Value::String(Some(Box::new(i.0)))
        }
    }
}
