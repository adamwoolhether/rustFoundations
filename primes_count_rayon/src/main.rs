fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

// Without rayon:
// Found 17984 primes in 1.6912987 seconds
/*fn main() {
    const MAX: u32 = 200_000;
    let now = std::time::Instant::now();

    let count = (2..MAX).filter(|n| is_prime(*n)).count();

    let duration = now.elapsed();
    println!("Found {count} primes in {} seconds", duration.as_secs_f32());
}*/

// With Rayon:
// Found 17984 primes in 0.18666038 seconds
/*fn main() {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    const MAX: u32 = 200_000;
    let now = std::time::Instant::now();

    let count = (2..MAX).into_par_iter().filter(|n| is_prime(*n)).count();

    let duration = now.elapsed();
    println!("Found {count} primes in {} seconds", duration.as_secs_f32());
}*/

// We can also use Rayon to manage our threads.
// Rayon uses a task oriented way to handle threads.
fn main() {
    use rayon::prelude::{IntoParallelIterator, ParallelIterator};

    rayon::scope(|s| {
        for i in 0..8 {
            let my_i = i;
            s.spawn(move |_| hello(my_i));
        }
    });
}

fn hello(n: u64) {
    use std::{thread::sleep, time::Duration};

    println!("Hello from thread{n}");
    sleep(Duration::from_secs(n));
    println!("Bye from thread {n}");
}
