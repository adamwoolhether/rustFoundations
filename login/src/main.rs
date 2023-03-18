use authentication::*;

fn user_accepted(role: &Role) {
    println!("You are logged in as a {role:?}."); // `:?` means to "print to debug expression
}

fn main() {
    let users = get_users();

    println!("Welcome to the Insecure Secure Server");
    println!("Enter your username:");
    let mut username = String::new();
    let mut password = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut username).unwrap();

    println!("Enter your password:");
    stdin.read_line(&mut password).unwrap();

    match login(&users, &username, &password) {
        None => {
            println!("{} is not a known user.", username.trim());
            println!("This is where we handle new users.");
        }
        Some(login_action) => login_action.do_login(user_accepted, |reason| {
            println!("Access denied!");
            println!("{reason:?}");
        }),
    }
}
