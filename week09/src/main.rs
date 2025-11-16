use std::{cell::Cell, ptr::null_mut};

fn main() {
    // Unsafe Rust
    // Throughout this course, you have been learning about what rust does to ensure memory safety

    // To do this Rust's compiler performs static analysis, which tends to be conservative when it comes
    // to error checking. It is better to reject some valid programs than to accept some invalid programs.
    // This can result in us writing code that is sound even though it raises an error

    // Safety vs Soundness
    // Soundness = the type system is correct

    // Safe abstractions over unsafe code
    // https://doc.rust-lang.org/stable/std/sync/struct.Arc.html#impl-Clone-for-Arc%3CT,+A%3E
    // https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#method.from_raw
    // What is the difference?

    // What does unsafe allow you to do?
    // Dereference a raw pointer
    // Call unsafe function
    // Access or modify mutable static variable
    // Implement unsafe trait
    // Access fields of union

    // What does unsafe not allow you to do?
    // turn off the borrow checker
    // disable other safety checks

    // Why is this helpful?

    // deref();
}

///
/// # Safety
/// this function actually shouldn't be run because it always results in an error
unsafe fn deref() {
    let mut x = Box::into_raw(Box::new(100));

    let num = 10;
    let y: *const i32 = &raw const num;

    x = null_mut();
    unsafe { let value = *x; }
}

struct Example<T> {
    cell: Cell<T>
}

// This is bad!
unsafe impl<T> Sync for Example<T> {}

struct MyBox<T> {
    inner: *const T,
}

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        Self { inner: &raw const x }
    }
}









// Best practice -> keep unsafe blocks small

// unsafe keyword
// declare the existence of contracts the compiler can't check
// declare that contracts have been upheld -> code is sound and has been checked
