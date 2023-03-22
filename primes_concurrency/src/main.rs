use std::sync::atomic::AtomicUsize;

// is_prime is an inefficient prime numbers counter used as an excessive CPU-bound workload.
fn is_prime(n: u32) -> bool {
    (2..=n / 2).all(|i| n % i != 0)
}

const MAX: u32 = 200_000;

// Using one thread
// Found 17984 primes in 1.4140562 seconds
/*fn main() {
    let mut count = 0;
    let now = std::time::Instant::now();
    for i in 2..MAX {
        if is_prime(i) {
            count += 1;
        }
    }
    let time = now.elapsed();
    println!("Found {count} primes in {} seconds", time.as_secs_f32());
}*/

// Using two threads
// Found 17984 prime numbers in the range 2..200000
// Execution took 1.0796026 seconds
/*fn main() {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let now = std::time::Instant::now(); // TIMER
    let t1 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (2..MAX / 2).filter(|n| is_prime(*n)).count(),
            std::sync::atomic::Ordering::Relaxed,
        );
    });
    let t2 = std::thread::spawn(|| {
        COUNTER.fetch_add(
            (MAX / 2..MAX).filter(|n| is_prime(*n)).count(),
            std::sync::atomic::Ordering::Relaxed,
        );
    });
    t1.join();
    t2.join();
    let duration = now.elapsed();
    println!(
        "Found {} prime numbers in the range 2..{MAX}",
        COUNTER.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("Execution took {} seconds", duration.as_secs_f32());
}*/

// Using many threads
// Found 17984 prime numbers in the range 2..200000
// Execution took 0.38733995 seconds
fn main() {
    const N_THREADS: u32 = 8;

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    // Hold thread handles
    let mut threads = Vec::with_capacity(N_THREADS as usize);

    // Generate all the numbers we want to check
    let group = MAX / N_THREADS;

    let now = std::time::Instant::now();

    for i in 0..N_THREADS {
        let counter = i;
        threads.push(std::thread::spawn(move || {
            let range = u32::max(2, counter * group)..(i + 1) * group;
            COUNTER.fetch_add(
                range.filter(|n| is_prime(*n)).count(),
                std::sync::atomic::Ordering::Relaxed,
            );
        }));
    }

    for thread in threads {
        let _ = thread.join();
    }

    let duration = now.elapsed();
    println!(
        "Found {} prime numbers in the range 2..{MAX}",
        COUNTER.load(std::sync::atomic::Ordering::Relaxed)
    );
    println!("Execution took {} seconds", duration.as_secs_f32());
}

// /////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first_hundred_primes() {
        // List obtained from: https://en.wikipedia.org/wiki/Prime_number
        let primes: Vec<u32> = (2..100).filter(|n| is_prime(*n)).collect();
        assert_eq!(
            primes,
            [
                2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
                83, 89, 97
            ]
        );
    }
}
