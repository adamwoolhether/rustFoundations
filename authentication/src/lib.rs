#[derive(PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
    Limited,
}

#[derive(PartialEq, Debug)]
pub enum DeniedReason {
    PasswordExpired,
    AccountLocked { reason: String }, // We can attach variables to individual entries.
}

#[derive(PartialEq, Debug)]
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

pub fn login(name: &str) -> Option<LoginAction> {
    // Option is a type that either does or doesn't have a value.
    // Its the closes thing to NULL in safe Rust.
    match name.to_lowercase().trim() {
        "adam" => Some(LoginAction::Accept(Role::Admin)),
        "mike" => LoginAction::standard_user(),
        "jake" => Some(LoginAction::Denied(DeniedReason::PasswordExpired)),
        "kevin" => Some(LoginAction::Denied(DeniedReason::AccountLocked {
            reason: "Call HR!".to_string(),
        })),
        _ => None,
    }
}

pub fn greet_user(name: &str) -> String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "adam"
}

#[cfg(test)] // Only compile next section for tests.
mod tests {
    use super::*;

    #[test] // Mark the function as a test to add it to Cargo's unit-test runner.
    fn test_greet_user() {
        assert_eq!("Hello Adam", greet_user("Adam"));
    }

    #[test]
    fn test_case_and_trim() {
        assert!(is_login_allowed("AdAM"));
        assert!(is_login_allowed("    AdAM\r\n  "));
    }

    #[test]
    fn test_login_fail() {
        assert!(!is_login_allowed("Alice"));
    }

    #[test]
    fn test_enums() {
        assert_eq!(login("Adam"), Some(LoginAction::Accept(Role::Admin)));
        assert_eq!(login("mike"), Some(LoginAction::Accept(Role::User)));
        assert_eq!(
            login("jake"),
            Some(LoginAction::Denied(DeniedReason::PasswordExpired))
        );
        assert_eq!(login("anonymous"), None);
        if let Some(LoginAction::Denied(DeniedReason::AccountLocked { reason: _ })) = login("kevin")
        {
            // Everything OK
        } else {
            panic!("Failed to read kevin");
        }
    }
}
