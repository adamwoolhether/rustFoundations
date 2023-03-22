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