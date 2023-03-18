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
    // Add a user.
    Add {
        /// Username.
        #[arg(long)]
        username: String,
        /// Password.
        #[arg(long)]
        password: String,
        /// Optional - mark as limited user.
        #[arg(long)]
        limited: Option<bool>,
        /// Optional - mark as admin.
        #[arg(long)]
        admin: Option<bool>,
    },
    /// Delete a user.
    Delete {
        /// Username.
        username: String, // Here we demonstrate not using the `#[arg]`
    },
}

type UserMap = HashMap<String, User>;

fn main() {
    let mut users = get_users();
    let cli = Args::parse(); // Tell Clap to start reading incoming command structure.
    match cli.command {
        Some(Commands::List) => {
            list_users(&users);
        }
        Some(Commands::Add {
            username,
            password,
            limited,
            admin,
        }) => {
            add_user(&mut users, username, password, limited, admin);
        }
        Some(Commands::Delete { username }) => {
            delete_user(&mut users, username);
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
        let action = format!("{:?}", user.action);
        let action = match user.action {
            LoginAction::Accept(..) => action.green(),
            LoginAction::Denied(..) => action.red(),
        };
        println!("{:<20}{:<20}", user.username, action);
    });
}

fn add_user(
    users: &mut UserMap,
    username: String,
    password: String,
    limited: Option<bool>,
    admin: Option<bool>,
) {
    if users.contains_key(&username) {
        println!("{username} already exists, aborting.");
        return;
    }
    let action = LoginAction::Accept(if limited.is_some() {
        // Giving an it statement as a parameter to a func.
        Role::Limited
    } else if admin.is_some() {
        Role::Admin
    } else {
        Role::User
    });
    let user = User::new(&username, &password, action);
    users.insert(username, user);

    save_users_file(users);
}

fn delete_user(users: &mut UserMap, username: String) {
    if !users.contains_key(&username) {
        println!("{username} doesn't exist, aborting");
        return;
    }
    users.remove(&username);
    save_users_file(users);
}
