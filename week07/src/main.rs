use crate::function_traits::{closure_example1, map_stuff};

mod function_traits;
mod macros;

fn main() {
    // Function Traits
    // map_stuff();
    closure_example1();

    let x = "word";
    let y = "more";
    let z = "another one";

    println!("Hello, World {x}, {y}, {}", z);

    // Macros
}
