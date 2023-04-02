use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

fn main() {
    // let mut readings = HashSetData::<usize, i32>::new();
    let mut readings = HashSetData::<usize, Data>::new(); // Use our custom data structure
    readings.add_reading(1, Data(-2));
    readings.add_reading(1, Data(3));
    readings.add_reading(1, Data(5));
    readings.add_reading(2, Data(1));
    println!("{readings:?}");
    readings.print_results();

    println!();
    println!();

    // Demonstrate Basic Example
    let mut store = StableVec::<String>::new();
    let a = store.push("A".to_string());
    let b = store.push("B".to_string());
    let c = store.push("C".to_string());
    println!("{:?}", store);

    store.remove(b);
    println!("{:?}", store.get(a));
    println!("{:?}", store.get(b));
    println!("{:?}", store.get(c));
    println!("{:?}", store[c]);
}

#[derive(Debug)]
struct Data(i32);

// HashSetData is a generic HashMap. Each entry stores its own
// vector of data.
#[derive(Debug)]
struct HashSetData<KEY, VALUE>
where
    // Restrict our data structure to data types that conform the the trait requirements.
    KEY: Eq + Hash + std::fmt::Display,
    VALUE: Debug + Sensor,
{
    data: HashMap<KEY, Vec<VALUE>>,
}

trait Sensor {
    fn reading(&self) -> i32;
}

impl Sensor for Data {
    fn reading(&self) -> i32 {
        self.0
    }
}

impl<KEY, VALUE> HashSetData<KEY, VALUE>
where
    KEY: Eq + Hash + std::fmt::Display,
    VALUE: Debug + Sensor,
{
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn add_reading(&mut self, key: KEY, reading: VALUE) {
        if let Some(entry) = self.data.get_mut(&key) {
            entry.push(reading);
        } else {
            self.data.insert(key, vec![reading]);
        }
    }

    fn print_results(&self) {
        for (key, value) in self.data.iter() {
            let sum: i32 = value.iter().map(|r| r.reading()).sum();
            let avg = sum / value.len() as i32;
            println!("key: {key}, avg: {avg}");
        }
    }
}

// /////////////////////////////////////////////////////////////////////////////////////////////////
// Basic Generic Data Store example:

// Struct with a generic type
#[derive(Debug)]
struct StableVec<T> {
    data: Vec<Option<T>>,
}

// Make a constructor. We declare `T` after the implementation, using it to fulfill the
// requirements of StableVec<T>.
impl<T> StableVec<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, item: T) -> usize {
        let id = self.data.len();
        self.data.push(Some(item));
        id
    }

    fn remove(&mut self, id: usize) {
        self.data[id] = None;
    }

    fn get(&self, id: usize) -> &Option<T> {
        &self.data[id]
    }
}

// Implement `Index` to support access via idx.
impl<T> Index<usize> for StableVec<T> {
    type Output = Option<T>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
