#[derive(Debug, PartialEq)]
pub struct User {
  pub uid: String,
  pub login: String,
  pub display_name: String,
  pub description: String,
}

impl User {
  pub fn new(uid: String, login: String, display_name: String, description: String) -> User {
    User {
      uid,
      login,
      display_name,
      description,
    }
  }
}