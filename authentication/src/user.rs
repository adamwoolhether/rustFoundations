use crate::{hash_password, LoginAction};
use serde::{Deserialize, Serialize}; // Refer to the top of the current crate's tree.

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub(crate) password: String, // `pub (crate)` makes the field public for this crate only.
    pub action: LoginAction,
}

impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(), // Convert a &str into a String.
            password: hash_password(password),
            action,
        }
    }
}
