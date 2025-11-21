
pub struct InsertUserParams<'a> {
  pub email: &'a str,
  pub username: &'a str,
  pub password_hash: &'a str,
}

