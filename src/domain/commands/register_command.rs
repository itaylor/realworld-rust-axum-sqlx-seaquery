use crate::http::dto::register::RegisterRequest;
use crate::model::values::email::Email;
use crate::model::values::username::Username;

pub struct RegisterCommand {
    pub(crate) username: Username,
    pub(crate) email: Email,
    pub(crate) password: String,
}

impl From<RegisterRequest> for RegisterCommand {
    fn from(req: RegisterRequest) -> Self {
        RegisterCommand {
            username: req.user.username,
            email: req.user.email,
            password: req.user.password,
        }
    }
}
