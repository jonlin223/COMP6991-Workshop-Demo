// Enums
// Defines a possible set of values for a type
// In Rust, we can directly place types inside the enum value
// We can even store different types in different enums including structs.
enum Options {
    OptionA,
    OptionB(String),
    OptionC(Foo),
}

struct Foo {
    x: i32,
    y: bool,
}

enum Pet {
    Dog,
    Cat,
    Mouse,
}

// So how does this relate to polymorphism
// We can implement a function that has different behaviours for each enum value
fn make_sound(pet: Pet) {
    match pet {
        Pet::Dog => println!("Woof"),
        Pet::Cat => println!("Meow"),
        Pet::Mouse => todo!(),
    }
}

// What is the advantage of this approach?
// What is the weakness of this approach?
