// Dynamic Dispatch and Trait Objects

// Rust chooses implementation of polymorphic operation at runtime

use std::error::Error;

fn trait_object() {
    // dyn keyword denotes use of dynamic dispatch
    // Creates what we call a trait object

    // usage of Box specifically provides ownership
    let pets: Vec<&dyn Pet> = vec![
        &Dog,
        &Cat
    ];

    walk_pets(pets);

    // Trait Object
        // Fat Pointer
            // One pointer to object data
            // one pointer to a vtable instance
        // when calling a method on a trait object, dynamic lookup process occurs to find correct polymorphic implementation
    
    // Can be used for return types as well.. for example Errors

    // Advantages


    // Disadvantages

}

struct MyError {
    message: String
}

fn example() -> Result<(), Box<dyn Error>> {
    todo!()
}

trait Pet {
    fn pet(&self);
    fn walk(&self);
}

struct Dog;
struct Cat;

impl Pet for Dog {
    fn pet(&self) {
        todo!()
    }

    fn walk(&self) {
        todo!()
    }
}

impl Pet for Cat {
    fn pet(&self) {
        todo!()
    }

    fn walk(&self) {
        todo!()
    }
}

fn walk_pets(pets: Vec<&dyn Pet>) {
    for pet in pets {
        pet.walk();
    }
}

// Monomorphis