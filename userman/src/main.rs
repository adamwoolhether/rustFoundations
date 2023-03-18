use authentication::*;
use clap::{Parser, Subcommand};
use std::collections::HashMap;

#[derive(Parser)]
#[command()] // The default command.
struct Args {
    #[command(subcommand)] // Defining additional commands, which are defined in the enum.
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all users.
    List,
}

type UserMap = HashMap<String, User>;

fn main() {
    let mut users = get_users();
    let cli = Args::parse(); // Tell Clap to start reading incoming command structure.
    match cli.command {
        Some(Commands::List) => {
            list_users(&users);
        }
        None => {
            println!("Run with --help to see instructions");
            std::process::exit(0);
        }
    }
}

fn list_users(users: &UserMap) {
    use colored::Colorize;
    println!("{:<20}{:<20}", "Username", "Login Action"); // Left align the field with pad of 20 chars.
    println!("{:-<40}", ""); // have a pad of `-` 40 chars wide.

    // Ignore the key with `_` in our call to for_each().
    users.iter().for_each(|(_, user)| {
        println!("{:<20}{:<20?}", user.username, user.action);
    })
}
