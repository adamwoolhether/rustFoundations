use once_cell::sync::Lazy;
use std::sync::Mutex;

struct MyType(usize);

impl MyType {
    fn new(n: usize) -> Self {
        Self(n)
    }
}

// NOTE: We can't always count on destructors for global types.
impl Drop for MyType {
    fn drop(&mut self) {
        println!("Drop");
    }
}

// Lazy will run initialization for our type the first time it's accessed. We can't forget to initialize it.
// Mutex ensures the value we're storing inside is free from data races, and that the contents are
// `Send+Sync`, allowing us to put anything there.
static SHARED: Lazy<Mutex<MyType>> = Lazy::new(|| Mutex::new(MyType::new(5)));

fn main() {
    println!("{}", SHARED.lock().unwrap().0)
}
