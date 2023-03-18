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
}

pub fn greet_user(name: &str) ->String {
    format!("Hello {name}")
}

pub fn is_login_allowed(name: &str) -> bool {
    name.to_lowercase().trim() == "adam"
}