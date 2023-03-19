use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
mod user;
pub use user::User; // export `user` mod from top-level.

pub fn build_users_file() {
    use std::io::Write;

    let users = get_users_old();
    let json = serde_json::to_string_pretty(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

pub fn get_users() -> HashMap<String, User> {
    let json = std::fs::read_to_string("users.json").unwrap();
    serde_json::from_str(&json).unwrap()
}

#[allow(dead_code)]
pub fn get_users_old() -> HashMap<String, User> {
    /*let mut result = HashMap::new();
    result.insert(
        "adam".to_string(),
        User::new("adam", "password", LoginAction::Accept(Role::Admin)),
    );
    result*/

    let mut users = vec![
        User::new("adam", "password", LoginAction::Accept(Role::Admin)),
        User::new("mike", "password", LoginAction::Accept(Role::User)),
        User::new(
            "jake",
            "password",
            LoginAction::Denied(DeniedReason::PasswordExpired),
        ),
        User::new(
            "kevin",
            "password",
            LoginAction::Denied(DeniedReason::AccountLocked {
                reason: "Contact HR!".to_string(),
            }),
        ),
    ];

    /*users
    .iter() //Create an iterator.
    .map(|user| (user.username.clone(), user.clone())) // Map to a tuple (username, user). We want a copy, not a pointer to user.
    .collect() // Collect infers the collection type from the function return.*/
    // Use drain to save memory:
    users
        .drain(0..)
        .map(|user| (user.username.clone(), user))
        .collect()
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
    Limited,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked { reason: String }, // We can attach variables to individual entries.
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

impl LoginAction {
    // An `associated function`, returns a variable of its type.
    // Associated funcs can interact with the type `Self` but
    // not with the content of any particular variable.
    #[allow(dead_code)]
    fn standard_user() -> Option<Self> {
        Some(LoginAction::Accept(Role::User))
    }

    // do_login is a `member function` allows interaction with the content
    // of any particular variable. `&self` means "provide a read-only
    // reference to myself", allowing the function to see the current value.
    // Two `function pointers` allow passing a func as a parameter and calling
    // them inside the do_login.
    pub fn do_login(&self, on_success: fn(&Role), on_denied: fn(&DeniedReason)) {
        match self {
            Self::Accept(role) => on_success(role),
            Self::Denied(reason) => on_denied(reason),
        }
    }
}

pub fn login(users: &HashMap<String, User>, username: &str, password: &str) -> Option<LoginAction> {
    // Option is a type that either does or doesn't have a value.
    // Its the closes thing to NULL in safe Rust.
    let username = username.trim().to_lowercase();
    let password = hash_password(password.trim());

    users
        .get(&username) // Returns the Option<User>
        .filter(|user| user.password == password) // Only keep Some(user) if the password matches.
        .map(|user| user.action.clone()) // Transform Some(user

    // Replaces:
    /*if let Some(user) = users.get(&username) {
        if user.password == password {
            Some(user.action.clone())
        } else {
            None
        }
    } else {
        // No user - return None
        None
    }*/

    // Using Vectors:
    /*users
    .iter()
    .find(|u| u.username == username && u.password == password)
    .map(|user| user.action.clone())*/
    // Replaces:
    /*if let Some(user) = users
        .iter()
        .find(|u| u.username == username && u.password == password)
    {
        Some(user.action.clone()) // Cloning conducts a deep-copy, retaining all interior information. Copy is generally faster, but anything with a String cannot be copied.
    } else {
        None
    }*/
}

pub fn hash_password(password: &str) -> String {
    let mut hasher = sha2::Sha256::new();
    hasher.update(password);
    format!("{:X}", hasher.finalize()) // `{:X}` means printing in hexadecimal. Prod system would want to add salt.
}

pub fn save_users_file(users: &HashMap<String, User>) {
    use std::io::Write;
    let json = serde_json::to_string_pretty(&users).unwrap();
    let mut f = std::fs::File::create("users.json").unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

#[cfg(test)] // Only compile next section for tests.
mod tests {
    use super::*;

    #[test] // Mark the function as a test to add it to Cargo's unit-test runner.
    fn test_enums() {
        let users = get_users();
        assert_eq!(
            login(&users, "Adam", "password"),
            Some(LoginAction::Accept(Role::Admin))
        );
        assert_eq!(
            login(&users, "mike", "password"),
            Some(LoginAction::Accept(Role::User))
        );
        assert_eq!(
            login(&users, "jake", "password"),
            Some(LoginAction::Denied(DeniedReason::PasswordExpired))
        );
        assert_eq!(login(&users, "anonymous", "none"), None);
        if let Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: _ })) =
            login(&users, "kevin", "password")
        {
            // Everything OK
        } else {
            panic!("Failed to read kevin");
        }
    }
}
