use std::{collections::HashMap, error::Error};

/* fn main() {
    // Vecs -> what is the difference between a vec and array?
    let mut array = [1,2,3,4,5]; // -> STACK

    // Let's try to mutate the array...
    // array = [1,2,3,4,5,6];

    // What about vecs now
    let mut v = vec![String::from("hello"), String::from("world")];
    let mut v1 = Vec::new();
    v1.push(10);

    // Hashmaps
    let h: HashMap<i32, String> = HashMap::new();

    // Iterators
    // iter vs into_iter

    // For Loops -> default behaviour

    // Derive macros

    // Let's create a struct...

    // Display -> General purpose printing
    // Debug -> Debug formatting
    let y = 1;
    println!("{y}");

    // Copy -> Bit-wise copy
    // Clone -> Duplicate underlying value

    // Why can some types be Copy but not Clone?

    let mut a = String::from("Hello");
    let b = a.clone();
    a = String::from("Hello World");
    println!("{}", b);
} */

fn main() -> Result<(), Box<dyn Error>> {
    let s = "Hello, World!";
    demo(s);

    Ok(())
}

fn do_something() -> Result<(), String>{
    Err(String::from("oh no!"))
}

fn demo(s: &str) {
    println!("{s}")
}

#[derive(Debug, Copy, Clone)]
struct Foo {
    x: i32,
    y: bool,
}

#[derive(Copy, Clone)]
struct Bar<'a> {
    f: &'a Foo,
}

