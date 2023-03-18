use authentication::is_login_allowed;

fn main() {
    println!("Welcome to the Insecure Secure Server");
    println!("Enter your username:");
    let mut input = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input).unwrap();
    if is_login_allowed(&input) {
        println!("Welcome, {input}");
    } else {
        println!("Sorry, {} is not allowed", input.trim());
    }
}
