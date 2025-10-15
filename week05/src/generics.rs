// Generics can be used with Traits
// Define a type using an abstract symbol, which can then be used interchangably

use std::{collections::HashMap, fmt::{Debug, Display}, hash::Hash};

// Generics

enum MyOption<T> {
    Some(T),
    None
}

// as soon as we want to do anything with generics, we have to bring in traits
fn print_list<T: Display>(list: Vec<T>) {
    for element in list {
        println!("{}", element);
    }
}

// Generics + Traits

trait Pet {
    fn identify(&self);
}

struct Dog {
    breed: String
}

impl Pet for Dog {
    fn identify(&self) {
        println!("This Pet is a {} dog", self.breed)
    }
}

struct Cat {
    breed: String
}

impl Pet for Cat {
    fn identify(&self) {
        println!("This Pet is a {} cat", self.breed)
    }
}

struct Mouse;


// So we can use our implementation of each trait
fn use_trait() {
    let dog = Dog { breed: String::from("Golden Retriever") };
    dog.identify();

    let cat = Cat { breed: String::from("Siamese") };
    cat.identify();

    // ---------------------------------- //
    use_generic(&cat);
    // example1(&dog, &cat);
    // example2(dog, cat);
}

// And we can also use it generically
fn use_generic<T: Pet>(pet: &T) {
    pet.identify();
}

// use_generic<T: Dog>
// use_generic<T: Cat>

// There are different ways to use trait bounds in functions
fn example1(pet1: &impl Pet, pet2: &impl Pet) {}
fn example2<T: Pet>(pet1: T, pet2: T) {}

// Multiple Bounds are supported
fn example3<T: Pet + Debug>(pet: T) {
    pet.identify();
    println!("{:?}", pet);
}

// Can also use generics with where clause
fn example4<T, U>(pet1: T, pet2: U)
where
    T: Pet + Display,
    U: Pet + Debug
{

}

// Advantage compared to enum usage?
// What is the weakness of this approach

// When working with trait bounds with generics, Rust determines types/chooses function implementation
// on compile time
fn static_dispatch() {
    let pets = vec![
        Dog { breed: String::from("Golden Retriever") },
        Cat { breed: String::from("Siamese") }
    ];


    for pet in pets {
        pet.identify();
    }
}



struct Pair<X: Eq + Hash, Y: Eq> {
    map: HashMap<X, Y>
}

impl<X: Eq + Hash, Y: Eq> Pair<X, Y> {
    fn new() -> Self {
        Pair { map: HashMap::new() }
    }

    fn add(&mut self, x: X, y: Y) {
        self.map.insert(x, y);
    }
}