use std::{sync::mpsc, thread};

// Demonstrating use of std lib to use channels.
fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let handle = thread::spawn(move || {
        loop {
            let n = rx.recv().unwrap();
            match n {
                1 => println!("Hi from worker thread"),
                _ => break,
            }
        }

        println!("Thread closed cleanly");
    });

    for _ in 0..10 {
        tx.send(1).unwrap();
    }
    tx.send(0).unwrap();

    handle.join().unwrap();
}

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrate use of broadcast channels with Tokio
/*use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::mpsc::{self, Receiver},
    time::sleep,
};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}

// Run the program and the 10 clients receiving an instruction to send a ping.
// Each client will receive it and report an Ack.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a broadcast channel and spawn 10 clients that subscribe to it.
    let (tx, _rx) = tokio::sync::broadcast::channel::<u32>(32);
    spawn(rpc_server());
    for _ in 0..10 {
        spawn(rpc_client(tx.subscribe()));
    }

    for _ in 0..10 {
        sleep(Duration::from_secs(1)).await;
        let _ = tx.send(1);
    }

    Ok(())
}
async fn rpc_client(mut rx: tokio::sync::broadcast::Receiver<u32>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;

    loop {
        let _n = rx.recv().await?;
        let message = serde_json::to_vec(&Request::Ping)?;
        stream.write_all(&message).await?;

        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let response: Response = serde_json::from_slice(&buf[0..n])?;
        match response {
            Response::Error => println!("Error!"),
            Response::Ack => println!("Ack"),
        }
    }
}

async fn rpc_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                // Create the response
                let mut response = Response::Error; // Create a default response, error.
                let request = serde_json::from_slice(&buf[0..n]); // Deserialize incoming buffer from byte slice.
                match request {
                    // Use `match` to cancel the socket if data was unreadable, read if result was well.
                    Err(..) => return,
                    Ok(request) => match request {
                        Request::Ping => response = Response::Ack, // If Ping is found, response with `Ack`.
                    },
                }

                // Send the response.
                let bytes = serde_json::to_vec(&response).unwrap();
                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }

    Ok(())
}*/

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrate use of channels with Tokio
/*use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::mpsc::{self, Receiver},
    time::sleep,
};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}

// Demonstrating the use of channels.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a "Multiple-producer, single consumer" channel.
    let (tx, mut rx) = mpsc::channel(32); // 32 buffer size.
    spawn(rpc_server());
    spawn(rpc_client(rx));

    for _ in 0..10 {
        sleep(Duration::from_secs(1)).await;
        let _ = tx.send(1).await;
    }
    let _ = tx.send(2).await;

    Ok(())
}

// cargo run -- --server
async fn rpc_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                // Create the response
                let mut response = Response::Error; // Create a default response, error.
                let request = serde_json::from_slice(&buf[0..n]); // Deserialize incoming buffer from byte slice.
                match request {
                    // Use `match` to cancel the socket if data was unreadable, read if result was well.
                    Err(..) => return,
                    Ok(request) => match request {
                        Request::Ping => response = Response::Ack, // If Ping is found, response with `Ack`.
                    },
                }

                // Send the response.
                let bytes = serde_json::to_vec(&response).unwrap();
                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }

    Ok(())
}

async fn rpc_client(mut rx: Receiver<u32>) -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;

    while let Some(n) = rx.recv().await {
        let message = serde_json::to_vec(&Request::Ping)?;
        stream.write_all(&message).await?;

        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let response: Response = serde_json::from_slice(&buf[0..n])?;
        match response {
            Response::Error => println!("Error!"),
            Response::Ack => println!("Ack"),
        }
    }

    Ok(())
}*/

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrating a TCP RPC server
/*use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    spawn,
    sync::mpsc::{self, Receiver},
    time::sleep,
};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Error,
    Ack,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Start the server if the `--server` flags is present. Clap is overkill for this.
    let args: Vec<String> = std::env::args().collect();
    if args.is_empty() {
        println!("You must run with either --server or --client")
    } else {
        match args[1].as_str() {
            "--server" => rpc_server().await?,
            "--client" => rpc_client().await?,
            _ => println!("You must run with either --server or --client"),
        }
    }
    Ok(())
}

// cargo run -- --server
async fn rpc_server() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8123").await?;

    loop {
        let (mut socket, address) = listener.accept().await?;
        spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                // Create the response
                let mut response = Response::Error; // Create a default response, error.
                let request = serde_json::from_slice(&buf[0..n]); // Deserialize incoming buffer from byte slice.
                match request {
                    // Use `match` to cancel the socket if data was unreadable, read if result was well.
                    Err(..) => return,
                    Ok(request) => match request {
                        Request::Ping => response = Response::Ack, // If Ping is found, response with `Ack`.
                    },
                }

                // Send the response.
                let bytes = serde_json::to_vec(&response).unwrap();
                socket
                    .write_all(&bytes)
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }

    Ok(())
}

//cargo run -- --client
async fn rpc_client() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8123").await?;
    let message = serde_json::to_vec(&Request::Ping)?;

    stream.write_all(&message).await?;

    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    let response: Response = serde_json::from_slice(&buf[0..n])?;
    match response {
        Response::Error => println!("Error!"),
        Response::Ack => println!("Ack"),
    }

    Ok(())
}*/

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrating a basic TCP Echo server.

/*use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Basic TCP Echo server:
    let listener = TcpListener::bind("127.0.0.1:8123").await?; // `?` to bail out with wrapped error if encountered.

    loop {
        // Wait for con & receive the address and a `TcpStream` when one arrives.
        // Note that we don't want to wait for this connection to finish to handle the next.
        let (mut socket, address) = listener.accept().await?;
        // Spawn a new green-thread to handle communication with this connection,
        // we do this with a closure within async.
        spawn(async move {
            // A buffer is used to read data and loop again to ready from keyboard.
            let mut buf = vec![0; 1024];
            loop {
                let n = socket // Move socket from parent task into new task, read and wait for it to received data.
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                // Exit if there's not data.
                if n == 0 {
                    return;
                }
                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
    }

    Ok(())
}*/

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrating basic async usage

// use tokio::{join, spawn, task::spawn_blocking};

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     hello(1).await; // Running a future, we need `await`.
//     The `join!` marco allows us to spawn multiple futures at once and wait for them.
// join!(hello(2), hello(3), hello(4));
// Ok(())
// }

/*async fn hello(n: u32) {
    println!("Hello {n}");

    if n < 10 {
        // async funcs can call (and await, join, etc) other async functions.
        hello_child(n * 10).await
        // To fire a task and not wait for it:
        // Note we can also optionally call wait on a spawn.
        /*spawn(hello_child(n * 10)).await;*/
    }
}

async fn hello_child(n: u32) {
    println!("Hello again {n}");

    // Demonstrate that even though our threads are joined, the whole
    // program will stall at the sleeping thread.
    /*std::thread::sleep(Duration::from_secs(1));*/

    // Tokio's thread pool is cooperatively managing green-thread scheduling.
    // So telling a thread to sleep will pause the entire Tokio runtime,
    // or the currently allocated thread (it's hard to tell which).
    // Here's how we solve it with Tokio, allowing the other threads
    // to continue, all of the sleep commands will finish together.
    /*tokio::time::sleep(Duration::from_secs(1)).await;*/

    // If we really do need to block and wait for something to finish,
    // we can do that by calling `spawn_blocking`. This will activate
    // a thread in the thread-pool, waiting for it to return. `await`
    // lets Tokio know that the green-thread is paused, and other green
    // threads can continue running, and when the blocking task returns,
    // then the green-thread returns.
    let _ = spawn_blocking(|| std::thread::sleep(Duration::from_secs(1))).await;
}*/

// Some other general notes for Tokio:
/*
You can't call an async function directly from a normal function.
You can call a regular function from an async function.
Futures aren't threads. If you have to block, let the runtime know - or you pause the world.
When you are in a blocking context, you can do whatever you want. You can even use rayon! Getting back into the async context is tricky, try to think in terms of independent operations that return results.
Recursion in green-thread land is hard.

*/
