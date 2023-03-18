pub struct User {
    pub username: String,
    password: String,
    action: LoginAction,
}

impl User {
    pub fn new(username: &str, password: &str, action: LoginAction) -> Self {
        Self {
            username: username.to_string(), // Convert a &str into a String.
            password: password.to_string(),
            action,
        }
    }
}

pub fn get_users() -> Vec<User> {
    vec![
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
    ]
}

#[derive(PartialEq, Debug, Clone)]
pub enum Role {
    Admin,
    User,
    Limited,
}

#[derive(PartialEq, Debug, Clone)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked { reason: String }, // We can attach variables to individual entries.
}

#[derive(PartialEq, Debug, Clone)]
pub enum LoginAction {
    Accept(Role),
    Denied(DeniedReason),
}

impl LoginAction {
    // An `associated function`, returns a variable of its type.
    // Associated funcs can interact with the type `Self` but
    // not with the content of any particular variable.
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

pub fn login(users: &[User], username: &str, password: &str) -> Option<LoginAction> {
    // Option is a type that either does or doesn't have a value.
    // Its the closes thing to NULL in safe Rust.
    let username = username.trim().to_lowercase();
    let password = password.trim();
    users
        .iter()
        .find(|u| u.username == username && u.password == password)
        .map(|user| user.action.clone())
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
