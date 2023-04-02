#[macro_use]
extern crate rocket;

use rocket::fs::NamedFile;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::tokio::io::AsyncWriteExt;
use std::net::TcpStream;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    username: String,
    password: String,
}

#[get("/")]
pub async fn login_page<'a>() -> NamedFile {
    NamedFile::open("login.html").await.unwrap()
}
/*fn index() -> &'static str {
    "Hello, world!"
}*/

#[post("/api/login", data = "<user>")]
pub async fn login(user: Json<Login>) {
    use rocket::tokio::io::{AsyncReadExt, AsyncWriteExt};
    use rocket::tokio::net::TcpStream;

    use authentication::*;
    let login_attempt = user.0;

    let mut stream = TcpStream::connect("127.0.0.1:8123").await.unwrap();
    let message = bincode::serialize(&login_attempt).unwrap();
    stream.write_all(&message).await.unwrap();

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await.unwrap();
    let response: Option<LoginAction> = bincode::deserialize(&buf[0..n]).unwrap();

    println!("{response:?}");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login_page, login])
}

// fn main() {
//     println!("Hello, world!");
// }
