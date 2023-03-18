#[cfg(test)] // Only compile next section for tests.
mod tests {
    use super::*;

    #[test] // Mark the function as a test to add it to Cargo's unit-test runner.
    fn test_greet_user() {
        assert_eq!("Hello Adam", greet_user("Adam"));
    }
}

pub fn greet_user(name: &str) ->String {
    format!("Hello {name}")
}

