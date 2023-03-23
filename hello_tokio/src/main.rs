use std::time::Duration;
use tokio::{join, spawn, task::spawn_blocking};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    hello(1).await; // Running a future, we need `await`.
                    // The `join!` marco allows us to spawn multiple futures at once and wait for them.
    join!(hello(2), hello(3), hello(4));
    Ok(())
}

async fn hello(n: u32) {
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
}

// Some other general notes for Tokio:
/*
You can't call an async function directly from a normal function.
You can call a regular function from an async function.
Futures aren't threads. If you have to block, let the runtime know - or you pause the world.
When you are in a blocking context, you can do whatever you want. You can even use rayon! Getting back into the async context is tricky, try to think in terms of independent operations that return results.
Recursion in green-thread land is hard.

*/
