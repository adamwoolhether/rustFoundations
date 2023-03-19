use crate::{DeniedReason, Role}; // super:: would also work.
use serde::{Deserialize, Serialize};

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
