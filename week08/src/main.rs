use std::{
    rc::Rc, sync::{Arc, Mutex}, thread
};

fn main() {
    // Fearless concurrency
    // What do we mean by this?
    // Rust tries to interpret as many concurrency errors as possible as compile time errors
    // Incorrect code should refuse to compile

    // What issues might we commonly run into when implementing concurrent programs?
    // Handling shared state across threads
    // Data races
    // Handdling locks, semaphores
    // Handle validityof data across threads, makes sure no references get dropped, become unavailable

    //count_single();
    count_multi();
}

#[derive(Debug, Clone)]
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

fn count_single() {
    let mut counter = Counter::new();

    for _ in 0..100 {
        counter.increment();
    }

    println!("{:?}", counter);
}

fn count_multi() {
    // Standard Thread Example
    /* let handle = thread::spawn(|| {
        for _ in 0..100 {
            counter.increment();
        }
        counter
    });

    let counter = handle.join().unwrap(); */

    // lifetimes with concurrency
    // Standard Scope Example

    /* thread::scope(|s| {
        s.spawn(|| {
            for _ in 0..100 {
                counter.increment();
            }
        });
    }); */

    // Start Multithreading and address issues

    // let counter = Mutex::new(Counter::new());
    let string = Rc::new(Mutex::new(String::from("Hello, World")));

    for _ in 0..10 {
        let string = Rc::clone(&string);
        thread::spawn(move || {
            string.lock().unwrap().push('!');
        });
    };

    // Shared State concurrency
}

// Spawn vs Scope
fn use_spawn() {
    let x = String::from("Hello, World!");
    thread::spawn(move || {
        println!("{x}");
    });
}

fn use_scope() {
    let x = String::from("Hello, World");

    thread::scope(|s| {
        s.spawn(|| {
            println!("{x}");
        });
    });
}

// Reminder
// Look at function signature of lock
// Maybe make Counter Clone but not Copy for example's sake





// Sync and Locking Primitives
// Mutex
// drop releases the lock automatically (make sure your code drops the lock...)
// Arc
// Provides shared ownership of data allocated in the heap
// look at the internals
// does not implement derefmut -> cannot mutate a shared reference -> requires interior mutability




// Send + Sync Traits
// Concurrenct traits
// Marker traits, don't have any functions that need to be implemented
// Tell us what types can be sent between threads
// Send -> type can be safely sent between threads
// Example of something that is Send
// Example of something that is not Send
// Sync -> reference of type can be safely shared between threads
// T is Sync iff &T is Send

// Show off compiler errors with Send, Sync

