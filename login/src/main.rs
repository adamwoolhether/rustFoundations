use authentication::{login, LoginAction};

fn main() {
    println!("Welcome to the Insecure Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();

    match login(&input) {
        LoginAction::Admin => println!("You are administrator"),
        LoginAction::User => println!("You have regular user rights"),
        LoginAction::Denied => println!("You are not allowed to login"),
    }
}
