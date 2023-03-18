use authentication::*;

fn user_accepted(role: &Role) {
    println!("You are logged in as a {role:?}."); // `:?` means to "print to debug expression
}

fn main() {
    println!("Welcome to the Insecure Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    match login(&input) {
        None => {
            println!("{} is not a known user.", input.trim());
            println!("This is where we handle new users.");
        }
        Some(login_action) => login_action.do_login(user_accepted, |reason| {
            println!("Access denied!");
            println!("{reason:?}");
        }),
    }
}
