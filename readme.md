## Strings
Rust has TWO types of string!
* `&str` is an immutable buffer of characters in memory. 
  * You usually use this for literals, such as "Herbert". 
  * You can refer to any String as an &str by borrowing it - with &my_string.
* `String` is an all-singing, all dancing buffered string designed for modification. 
  * Internally, String is a buffer of characters with the length stored. 
  * Changing a String updates or replaces the buffer.

## Borrow Checker
* Rust assumes that all programs may be multi-threaded.
* Only one thing at a time may *ever* have mutable (write) access to a variable.
* Any number of things may have *read only* access to a variable---but only if nothing can currently write to it.

#### Advice
* **Keep a really obvious data-path through which data may be modified.**
* Don't use Rust as an object-oriented language. It isn't one. You won't have much trouble if you combine simple data types and retain a store associating types with types---like a relational database.
* You *will* have a miserable time if you implement a bunch of functionality-specific traits, mix and match them, and store them in a giant, C++ style common object store. You'll tie yourself in knots trying to match traits, extract the information from *that* trait, and working with lots of borrowed trait data.

## Fearless Concurrency
* You can't use a variable after it's gone (moved or destroyed).
* You can't accidentally invalidate an iterator (by changing what its iterating).
* You can't accidentally destroy a variable while you are still using it somewhere else.
* You can't make a data race condition, because synchronized access makes it impossible. *Unless you mark your code as unsafe and promise Rust that you know what you're doing*. We'll look at this one when we get to concurrency.

## Some Key Rust Designs
* Everything is immutable by default unless you tell Rust otherwise.
* Making a variable, and then making a *new* variable for the next stage is good practice---functional programming style.
* Assignments are *move* by default...

## Move
* Rust _moves variables_ by default, except for some small primitives that implement the `Copy` struct. This has two implications:
  * The variable no longer exists in its previous state.
  * The recipient _has ownership_ of the variable.

To demonstrate, see this code that _will not compile:_
```rust
fn do_something(s: String) { // String is NOT a copyable type.
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    do_something(s); // Move `s`, passing ownership of it to `do_something`.
    do_something(s); // `s` doesn't exist anymore, so this is invalid.
}
```
We could fix it like this, by making a copy of `s` and move that into `do_something`:
```rust
fn do_something(s: String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    do_something(s.clone());
    do_something(s);
}
```
BUT, this is wasteful. We're better off passing a _reference_ and borrowing the parameter:
```rust
fn do_something(s: &String) {
    println!("{s}");
}

fn main() {
    let s = "Hello".to_string();
    do_something(&s);
    do_something(&s);
}
```

### Demonstrate how ownership and destruction work:
```rust 
struct Data(usize);

impl Drop for Data {
    fn drop(&mut self) {
        println!("Data object {} is being destroyed", self.0);
    }
}

fn do_something(d: Data) {
    println!("Hello data #{}", d.0);
}

fn main() {
    let data = Data(1);
    do_something(data);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Program ending");
}
```

Returning non-copy types also results in a _move_:
```rust 
struct Data(usize);

impl Drop for Data {
    fn drop(&mut self) {
        println!("Data object {} is being destroyed", self.0);
    }
}

fn do_something(d: Data) -> Data {
    println!("Hello data #{}", d.0);
    d
}

fn main() {
    let data = Data(1);
    let data = do_something(data);
    do_something(data);
    std::thread::sleep(std::time::Duration::from_secs(5));
    println!("Program ending");
}
```

### Builder Pattern
This helps us avoid borrow checker issues. We _move_ out of each step, never borrowing. Since we don't borrow, the borrow checker doesn't look at our code.  
We don't need this in other langs(go, java, etc) because the GC helps us abstract variable: the variable doesn't represent itself, it's a pointer stored in memory.  
In Rust, we need to keep track of who has ownership of the variable. 
* Code smell: We're passing variables all over that need to be changed -- design problem.
```rust 
#[derive(Default)]
struct MyBuilder {
    a: bool,
}

impl MyBuilder {
    fn with(mut self, a: bool) -> Self {
        self.a = a;
        self
    }

    fn build() -> Self {
        Self::default().with(true)
    }
}

fn main() {
    let _x = MyBuilder::build();
}
```
And an example of what DOES NOT work:
```rust
#[derive(Default)]
struct MyBuilder {
    a: bool,
}

impl MyBuilder {
    fn with(&mut self, a: bool) -> Self {
        self.a = a;
        self
    }

    fn build() -> Self {
        Self::default().with(true)
    }
}

fn main() {
    let x = MyBuilder::build();
}
```

## Iterators and Borrow Checker
By creating iterators, we are _borrowing_ the entire vector, so we can't change an element in the vector while we iterate it.  
**You can't have mutable access to anything while something else has any access to it.**
This won't work:
```rust
struct Node {
    parent: usize,
}

fn main() {
    let mut nodes = vec![
        Node{parent: 0},
        Node{parent: 1},
        Node{parent: 2},
    ];

    nodes.iter().enumerate().for_each(|(i, node)| {
        if node.parent == 1 && i > 0 {
            nodes[i-1].parent = 0;
        }
    });
}
```
To make it work, we'd have to change the iterator code with a simple loop:
```rust
for i in 1..nodes.len() {
    if nodes[i].parent == 1 {
        nodes[i-1].parent = 0;
    }
}
```
## Lifetimes
Lifetimes prevent using a pointer after the item being pointed to is no longer available. It's needed because, unlike GC langs, we need prevent lifetime issues.

This func passes a _reference_ to an i32, main _borrows_ `x`.
```rust
fn do_something(x: &i32) {
    println!("{x}"); 
}

fn main() {
    let x = 12;
    do_something(&x);
}
```
This is a syntactically more sound version of the following:
```rust
fn do_something<'a>(x: &'a i32) {
    println!("{x}"); 
}
```

### We still have to annotate lifetimes for functions that return and take more than one reference:
```rust 
fn get_x<'a, 'b>(x: &'a i32, _y: &'b i32) -> &'a i32 {
    x
}

fn main() {
    let a = 1;
    let b = 2;
    let _ = get_x(&a, &b);
}

// This doesn't work:
/*
fn get_x(x: &i32, _y: &i32) -> &i32 {
    x
}
*/
```

### Lifetimes for Structures
We can keep a reference for later use if we provide a lifetime annotation:
```rust 
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a Cat
}

fn main() {
    let cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    for cat in cats.iter() {
        feeders.push(CatFeeder{ cat })
    }
}
```

The following demonstrates how Rust allows the borrow-checker and lifetime checker to handle in-vector mutable borrowing and keep a mutable reference around:
```rust
struct Cat(String);

struct CatFeeder<'a> {
    cat: &'a mut Cat
}

impl Cat {
    fn feed(&mut self) {
        self.0 = format!("{} (purring)", self.0);
    }
}

impl<'a> CatFeeder<'a> {
    fn feed(&mut self) {
        self.cat.feed();
    }
}

fn main() {
    let mut cats = vec![
        Cat("Frodo".to_string()),
        Cat("Bilbo".to_string()),
        Cat("Pippin".to_string()),
    ];
    
    let mut feeders = Vec::new();
    for cat in cats.iter_mut() {
        feeders.push(CatFeeder{ cat })
    }
    
    feeders.iter_mut().for_each(|f| f.feed());
}
```
With `impl<'a> CatFeeder<'a>`, we say "implement CatFeeder for lifetime 'a"

### Garbage Collection in Rust?
To pass and keep references, Rust has opt-in _reference counting_, a pointer that counts how many times it's being held, deleting it only when nobody is looking at it.  
The following code demonstrates the case of needing read-only, safe, garbage collected pointers. We'll create a reference counted cats, giving their owners a reference to the cat.
```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
  cat: Rc<Cat>
}

fn main() {
  let mut cats = vec![
    Rc::new(Cat("Frodo".to_string())),
    Rc::new(Cat("Bilbo".to_string())),
    Rc::new(Cat("Pippin".to_string())),
  ];
  
  let mut owners = Vec::new();
  for cat in cats {
    owners.push(CatOwner{ cat: cat.clone() }); // We're actually cloning the `Rc`, incrementing the ownership counter.
  }
  
  for owner in owners {
    println!("{}", owner.cat.0)
  }
}
```
The `Rc` counter is designed to be cloned, it's fast! It's useful when we need _read-only_ references to a type, ensuring that the pointer
lives long enough to remain valid. The code below is the same, showing how we've "defeated" the lifetime system:
```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
    cat: Rc<Cat>
}


fn main() {
    let mut owners = Vec::new();
    {
        let mut cats = vec![
            Rc::new(Cat("Frodo".to_string())),
            Rc::new(Cat("Bilbo".to_string())),
            Rc::new(Cat("Pippin".to_string())),
        ];
        
        for cat in cats {
            owners.push(CatOwner{ cat: cat.clone() });
        }
    }
    
    for owner in owners {
        println!("{}", owner.cat.0)
    }
}
```

Add a `Drop` handler to show when cats cease to exist:
```rust
use std::rc::Rc;

struct Cat(String);

struct CatOwner {
  cat: Rc<Cat>
}

impl Drop for Cat {
  fn drop(&mut self) {
    println!("{} was dropped!", self.0)
  }
}

fn main() {
  {
    let mut owners = Vec::new();
    {
      let mut cats = vec![
        Rc::new(Cat("Frodo".to_string())),
        Rc::new(Cat("Bilbo".to_string())),
        Rc::new(Cat("Pippin".to_string())),
      ];
      
      for cat in cats {
        owners.push(CatOwner{ cat: cat.clone() });
      }
    }
    for owner in owners.iter() {
      println!("{}", owner.cat.0)
    }
  }
  println!("Program end!")
}
```

### If we NEED to store Mutable Pointers
We can do this with the _interior mutability_ pattern. Below, `Cat` appears immutable and accessors remain immutable by using `self` rather than `mut self`.  
`RefCell` implements locking to ensure that `borrow_mut` and `borrow` won't break memory safety guarantees.
```rust
use std::rc::Rc;
use std::cell::RefCell;

struct Cat {
  name: RefCell<String>
}

impl Cat {
  fn new(name: &str) -> Self {
    Self {
      name: RefCell::new(name.to_string())
    }
  }
}

struct Owner {
  cat: Rc<Cat>
}

impl Owner {
  fn feed(&self) {
    let mut name_borrow = self.cat.name.borrow_mut();
    *name_borrow += " (purring)";
  }
}

fn main() {
  let cats = vec![
    Rc::new(Cat::new("Frodo")),
    Rc::new(Cat::new("Bilbo")),
    Rc::new(Cat::new("Pippin")),
  ];

  let mut owners = Vec::new();
  for cat in cats.iter() {
    owners.push(Owner{ cat: cat.clone() });
  }

  for owner in owners.iter() {
    owner.feed();
  }

  for owner in owners.iter() {
    println!("{}", owner.cat.name.borrow());
  }
}
```

## OOP Patterns
Coming from an OOP background, we may be tempted to do something like this:
This **wont work**:
```rust
struct Organization {
    pub people: Vec<Person>,    
}

struct Person {
    pub resources: Vec<Resource>,
}

impl Person {
    fn give_resource(&mut self, name: &str, org: &mut Organization, recipient: usize) {
        if let Some((idx, resource)) = self.resources.iter().enumerate().find(|(_, item)| name == item.name) {
            self.resources.remove(idx);
            org.people[recipient].resources.push(resource.clone());
        }
    }
}

#[derive(Clone)]
struct Resource {
    pub name: String,
}

fn main() {
    let mut org = Organization {
        people: vec![
            Person { resources: vec![ Resource { name: "Stapler".to_string() } ]},
            Person { resources: Vec::new() },
        ]
    };
    org.people[0].give_resource("Stapler", &mut org, 1);
}
```

But this violates the **golden rule of the borrow checker**: _we try to borrow something mutably and also have mutable access to it_. But with Rust, we have to break the problem down into smaller parts.  
Let's move the function, making it an _organization_ function: meaning we only require one ownership while the organization retains ownership of people, allowing us to modify them.
```rust
struct Organization {
  pub people: Vec<Person>,
}

// The Organization owns the workers and their resources. A single mutable borrow gives us the ability to give
// `Organization` orders, without needing to index the `Organization`.
impl Organization {
  // calling `move_resource` we break the operation down into two steps:
  // 1. Call `take_resource` on a person, encouraging us to check that the person has the resource.
  // 2. When sure that the person has the resource, move it to the organization. `remove` will
  // return the removed structure, so we can hand it off with move.
  // 3. Now we're sure that we only got a copy of the resource, we can move it to the new recipient.
  fn move_resource(&mut self, from: usize, to: usize, name: &str) {
    if let Some(resource) = self.people[from].take_resource(name) {
      self.people[to].give_resource(resource);
    }
  }
}

struct Person {
  pub resources: Vec<Resource>,
}

impl Person {
  fn take_resource(&mut self, name: &str) -> Option<Resource> {
    let index = self.resources.iter().position(|r| r.name == name);
    if let  Some(index) = index {
      let resource = self.resources.remove(index);
      Some(resource)
    } else {
      None
    }
  }
  
  fn give_resource(&mut self, resource: Resource) {
    self.resources.push(resource);
  }
}

struct Resource {
  pub name: String,
}

fn main() {
  let mut org = Organization {
    people: vec![
      Person { resources: vec![ Resource { name: "Stapler".to_string() }]},
      Person { resources: Vec::new() },
    ]
  };
  org.move_resource(0, 1, "stapler");
}
```
We would want proper err handling for a production scenario: see [here](https://github.com/thebracket/ArdanUltimateRustFoundations/blob/main/day2/hour2/oop.md#cleaning-up-and-adding-error-handling).

## RAII: Resource Acquisition is Initialization
We should implement `Drop` for acquiring finite resources to guarantee that we relinquish our hold on the program.
Ex:
```rust
struct Droppable;

impl Drop for Droppable {
  fn drop(&mut self) {
    println!("Destruction")
  }
}
```

## Global Variables
Rust doesn't support simple global vars.
* You can't be sure when/where changes to global vars come from. This makes it impossible for borrow checker to do its job.
* Rust assumes multi-threaded environment, it must assume the worst: that changes can come from anywhere, anytime.  
This won't work:
```rust
 let shared = 5;

fn main() {
  println!("{shared}");
}
```
### Constants
We need to use a **Constant**:
```rust
const SHARED: usize = 5;

fn main() {
  println!("{SHARED}");
}
```
Constants can be global or public global, they are immutable

### Static Vars
We can also use global vars with `static`, they can be mutable or not, although the compiler does not like it when we make them mutable!!!
```rust
static SHARED: usize = 5;
// static mut SHARED: usize = 5;

fn main() {
    println!("{SHARED}");
}
```

### Unsafe
If we're feeling dangerous, we can make static vars `unsafe`. But we likely _shouldn't_ do this!
```rust
static mut SHARED: usize = 5;

fn main() {
    unsafe {
        SHARED += 1;
        println!("{SHARED}");
    }
}
```

### Safely share types with Interior Mutability
As discussed in section on lifetimes, we can use interior mutability patterns to safely share data. The constructor _must_ be a constant function.
```rust
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static SHARED: AtomicUsize = AtomicUsize::new(5);

fn main() {
        println!("{}", SHARED.load(Ordering::Relaxed));
}
```
We can also use a synchronization primitive:
```rust
use std::sync::Mutex;

static SHARED: Mutex<usize> = Mutex::new(5);

fn main() {
  println!("{}", *SHARED.lock().unwrap());
}
```

### Lazy Singletons
See the `global_lazy_init` example

## Synchronization Primitives
Mutex might not always be the type of synchronization we want.

### Understanding Locking
Let's talk about this example:
```rust
use std::sync::Mutex;

struct MyType(usize);

impl MyType {
    const fn new(n: usize) -> Self { // <-- Notice we've added a constant constructor
        Self(n)
    }
}

static SHARED: Mutex<MyType> = Mutex::new(MyType::new(5));

fn main() {
    println!("{}", SHARED.lock().unwrap().0);

  let mut lock = SHARED.lock().unwrap();
  lock.0 += 1;
  println!("{}", lock.0);

  // This would create a deadlock!!! We locked twice! Rust doesn't provide safety against runtime deadlocks
  // println!("{}", SHARED.lock().unwrap().0);
}
```
`SHARED.lock().unwrap()` has two stages: 
1. `lock()` for exclusive access to the interior variable. We can mutate it.
2. `unrawp()` catches any errors that may occur.
`let mut lock = SHARED.lock().unwrap();` will give us mutable access to change the global variable.

The above code block also includes an example of a deadlock. We can mitigate this by using `Drop` to release the lock, done by simply dropping `lock` out of scope when we're done with it:
```rust
fn main() {
  { 
    let mut lock = SHARED.lock().unwrap(); 
    lock.0 += 1;
  }
  
  println!("{}", SHARED.lock().unwrap().0);
}
```
Or we can manually drop the lock:
```rust
fn main() {
  let mut lock = SHARED.lock().unwrap();
  lock.0 += 1;
  std::mem::drop(lock);
  println!("{}", SHARED.lock().unwrap().0);
}
```

### Types of Lock
Rust has two popular locking primitives. We've seen `Mutex`, and there is also `RwLock`. Similar usage:
```rust
use std::sync::RwLock;

struct MyType(usize);

impl MyType{
  const fn new(n: usize) -> Self { // <-- Using a constant constructor.
    Self(n)
  }
}

static SHARED: RwLock<MyType> = RwLock::new(MyType::new(5));

fn main() {
  for _ in 0..10 {
    std::thread::spawn(|| {
      let read_lock = SHARED.read().unwrap();
      println!("The value of SHARED is {}", read_lock.0)
      // To drop the read lock:
      // std::mem::drop(read_lock);
    });
    
    std::thread::spawn(|| {
      let mut write_lock = SHARED.write().unwrap();
      write_lock.0 += 1;
    });
    
    std::thread::sleep(std::time::Duration::from_secs(5));
  }
}
```

## New Types
See the `new_types` section.

### ToString trait
If we're sick of typing `to_string()` everywhere, we can implement this trait:
```rust
fn take_my_text<S: ToString>(text: S) {
  let _s = text.to_string();
  // Work with the string
}

fn main() {
  take_my_text("Hello");
  take_my_text("Hello".to_string());
  take_my_text(String::new());
  let n = 5;
  take_my_text(n);
}
```

## Traits
Here we demonstrate a `Cat` that implements an `Animal` trait that has a default fn if it's not explicitly implemented.
```rust
trait Animal {
    // fn make_noise(&self);
    fn make_noise(&self) {
      println!("Who knows what noise I make?")
    }

}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
}

struct Tortoise;

impl Animal for Tortoise {}

fn main() {
  let cat = Cat{};
  cat.make_noise();
  let tortoise = Tortoise{};
  tortoise.make_noise();
}
```

Note that traits cannot contain any data, only functions. But you an reference internals of a type:
```rust
trait Animal {
  // fn make_noise(&self);
  fn make_noise(&self) {
    println!("Who knows what noise I make?")
  }

}
struct Cat {
    noise: String
}

impl Animal for Cat {
    fn make_noise(&self) {
        println!("{}", self.noise);
    }
}

fn main() {
    let cat = Cat{ noise: "meow".to_string() };
    cat.make_noise();
}
```

### Generic Funcs and Trais dependencies.
```rust
trait Animal {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}

struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
}

struct Tortoise;

impl Animal for Tortoise {}

// Here's a generic func that requires our trait, we can pet all animals:
// fn pet<A: Animal>(animal: A)
// {
//   animal.make_noise()
// }

// Second trait:
trait Tame {}

// Pretend that cats are tame:
impl Tame for Cat{}

// Now we can prevent the petting of animals that aren't tame!
fn pet<A: Animal + Tame>(animal: A) {
  animal.make_noise()
}

fn main() {
  let cat = Cat{};
  let tortoise = Tortoise{};
  pet(cat);
  pet(tortoise);
}
```

### Polymorphic Traits
How would we store variables that all implement a trait into the same collection/vector??
This won't work:
```rust
fn main() {
    let cat = Cat{};
    let tortoise = Tortoise{};
    
    let animals = vec![cat, tortoise];
}
```
We need to use `Box`, and explicitly tell Rust to turn on _dynamic dispatch_.
`dyn` means that "the actual type may change", ie: it's dynamic.
```rust

fn main() {
  let cat = Cat{};
  let tortoise = Tortoise{};
  
  let mut animals: Vec<Box<dyn Animal>> = Vec::new(); // Note `dyn` here.
  animals.push(Box::new(cat));
  animals.push(Box::new(tortoise));
  for animal in animals.iter() {
    animal.make_noise();
  }
}
```

### Making Traits require other traits
Let's make every animal support debug:
```rust
trait Animal: std::fmt::Debug + std::fmt::Display { // Added a second requirement as well.
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
}

#[derive(Debug)]
struct Cat;

#[derive(Debug)]
struct Tortoise;

// Our second requirement above means we have to make every type printable.
impl std::fmt::Display for Cat {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Cat")
  }
}

impl std::fmt::Display for Tortoise {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "Tortoise")
  }
}

// This allows us to debug-print every animal in the loop:
fn main() {
  for animal in animals.iter() {
    println!("{:?}", animal);
    animal.make_noise();
  }
  
  // Second requirement above allows us to print an animal like a primitive:
  for animal in animals.iter() {
    println!("{animal}");
    animal.make_noise();
  }
}
```

### If you need to know the concreted type:
```rust 
use std::any::Any;

trait Animal: std::fmt::Debug + std::fmt::Display + Any {
    fn make_noise(&self) {
        println!("Who knows what noise I make?")
    }
    
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct Cat;

impl Animal for Cat {
    fn make_noise(&self) {
        println!("Meow")
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Debug)]
struct Tortoise;

impl Animal for Tortoise {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl std::fmt::Display for Cat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Cat")
    }
}

impl std::fmt::Display for Tortoise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Tortoise")
    }
}

fn main() {
    let cat = Cat{};
    let tortoise = Tortoise{};

    let mut animals: Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(cat));
    animals.push(Box::new(tortoise));
    
    for animal in animals.iter() {
        if let Some(cat) = animal.as_any().downcast_ref::<Cat>() {
            println!("We have access to the cat");
        }
        println!("{animal}");
        animal.make_noise();
    }
}
```

## Constants Details
### Compile-Dependent Constants
To adjust compile-time declaration of contants:
```rust 
// Based on release:
#[cfg(debug)]
const A: usize = 3;
#[cfg(release)]
const A: usize = 4;

// Based on OS:
#[cfg(target_family = "unix")]
const A: usize = 3;
#[cfg(not(target_family = "unix"))]
const A: usize = 4;
```

### Constant Functions
Add `const` to functions to have them execute at compile time:
```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const A: usize = add(4, 6);

fn main() {
    println!("{A}");
}
```
And we can use the constant function with dynamic inputs:
```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const A: usize = add(4, 6);

fn main() {
    let mut i = 5;
    i += 3;
    println!("{}", add(A, i));
    println!("{A}");
}
```
A more complicated example:
```rust
const fn loopy() -> usize {
  let mut n = 1;
  let mut i = 0;
  while i < 20 {
    n += i;
    i += 1;
  }
  n
}

const A: usize = loopy();

fn main() {
    println!("{A}");
}

// Note that `loopy` wouldn't work if we used a for loop as such:
/*const fn loopy() -> usize {
  let mut n = 1;
  for i in 0..20 {
    n += i;
  }
  n
}*/
```

Some things we can't do at compile time:
* Use floating point numbers, except as direct constants. Functions using floating points won't work.
* Use iterators.
* Connect to external data sources _other than_ files. (`include_str!` and `include_bytes!` can embed files in our binary)

## Macros
Let us change the language's syntax. Two types:
* Declarative
* Procedural

Declarative are more simple, and we'll only cover them here.

Using macros are always defined with `!`.

## Benchmarks
Quick and dirty way using `Instant` and `Duration`.
```rust
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    for j in 0 .. 1_000 {
        i += j*j;
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {} nanos", elapsed.as_nanos());
    println!("{i}");
}
```

### Embedding benchmarks in tests:
`criterion` test suite will allow this, but we need the `nightly` toolchain:
```rustup install nightly
cargo init bench --lib
```
**See bench lib**
And we can run the bench with:
```cargo +nightly bench```

## Rust Safe Guarantees
#### Memory
* Guarantees you won't use-after free, buffer overrun, dangling pointer, or iterator invalidation.
* Does NOT guarantee prevention of memory leaks.
#### Thread Safety
* It prevents data-races.
* Does NOT prevent creation of deadlocks via Mutex misuse.
#### Type Safety
* Done when explicitly using `NewTypes` (or similar)
#### `Unsafe` Tag
* Needed when you talk directly with hardware or call programs through FFI.
#### Audits
* When used in production, be sure to use auditing:
```
cargo install cargo-audit
run cargo audit
```