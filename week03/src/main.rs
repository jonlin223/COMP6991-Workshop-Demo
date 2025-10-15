use std::ops::Deref;

fn main() {
    // Why ownership?
    // How does C use memory management?
    // How does Python do memory management?
    // How does Rust do memory management?

    // What is the issue with Rust's memory management system?
    // Why is ownership the solution?
    // Double Free
    let mut x = String::from("Hello, World!");
    // println!("{x}");

    // Double Free
    // Dangling pointer

    // Copy vs Clone... again
    // Stack vs Heap... again

    // References... using values without transferring ownership
    // why can we only have one mutable reference?
    // draw this...

    // REFRENCES IN RUST ARE ALWAYS SAFE TO USE.

    let mut buffer = vec![1,2,3,4,5];
    let i = buffer.get(1);
    println!("{:?}", i);

    // Slices
    // &str type
    // Slice contains pointer to start of data and length of data
    let s = String::from("This is a test string!");
    let s1 = &s[0..4];
    let s2 = &s;

    let slice = &buffer[1..2];

    transfer_onwerhip(s2);

    // See deref trait
    string_param(s1);
    string_param(s2);

    let x = "hello";
    // Lifetimes
    let x = {
        let y = "aasdkasd asldalk";
        lifetimes_basic(x, y)
    };
    println!("{x}");



}

fn transfer_onwerhip(x: &str) {
    println!("{x}");
}

fn drop_integer(x: i32) {
    drop(x);
}

fn string_param(x: &str) {
    let s = String::from("hi");
}

fn lifetimes_basic<'a>(x: &'a str, y: &'a str) -> &'a str {
    y
}

fn lifetimes_advanced() {
    
}