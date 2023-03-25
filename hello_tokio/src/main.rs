use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    spawn,
};
// use tokio::{join, spawn, task::spawn_blocking};

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
}

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Demonstrating basic async usage

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
