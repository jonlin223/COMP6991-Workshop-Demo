// Function Pointers
// You can pass functions as arguments to functions in Rust!

fn do_something(i: &String) -> bool {
    i.contains("Hello")
}

fn call_function(f: fn(&String) -> bool) {
    let x = String::from("Hello, World!");
    let value = f(&x);
}

fn fake_main() {
    // So this works...
    call_function(do_something);
}

// Is this useful...
// sure

pub fn map_stuff() {
    let v = vec![
        "hi".to_owned(),
        "HELLO".to_owned(),
        "Hello".to_owned(),
        "Hello, World!".to_owned(),
        "World".to_owned(),
    ];
    let v1: Vec<bool> = v.iter().map(do_something).collect();
    println!("{:?}", v1)
}

// But we also have a better method... Closures
// Closures differ from function pointers in their ability to capture values from their scope
pub fn closure_example1() {
    let x = 1;

    // X is captured by the environment -> closure implements Fn because captured variable only used as a immutable reference
    let closure = || {
        println!("{x}");
    };

    println!("{x}");

    closure();
    closure();
}

// Question... What happens if we move a value into a closure
// Example 1 only uses a reference to the value to print it, we can obviously still use it afterwards according to rust's
// ownership rules
pub fn closure_example2() {
    let mut x = String::from("Hello, World!");
    let z = "test";

    // x moved into closure
    let mut closure = || {
        let y = x;
        println!("{z}");
    };

    // Therefore cannot be used here
    println!("{x}");

    // What do you think happens to x?

    // FnOnce?
    // And cannot be used several times since running the closure would drop the value (x/y)
    closure();
    closure();
}

// Function Traits
// FnOnce
// FnMut
// Fn
// Do you notice anything familiar about this trio? -> Map to rust ownership rules

// Closure's trait implementation is determined by its use of a captured value
// Function Trait defines how it can be used
// FnOnce can only be used once -> Why?
// FnMut can be used many times but not simultaneously -> Why?
// Fn can be used many times simultaneously -> Why?

// When using function traits as parameters, which one should we use?

fn for_each<T, F>(v: Vec<T>, mut f: F)
where
    F: FnOnce(T),
{
    for t in v {
        f(t);
    }
}

// FnOnce -> a closure you can run once -> ownership
// FnMut -> a closure you can run as many times as you want non concurrenly -> mutable references
// Fn -> a closure you can run as many times as you want concurrently -> reference

// As we move from FnOnce -> FnMut -> Fn, function traits become MORE restrictive
// Since less closures implement Fn than FnMut than FnOnce
// when designing functions, we always want to use the least restrictive function trait possible

fn test() {
    let mut x = String::from("Hello, World");
    let mut closure = |t: &str| {
        let y = &mut x;
        y.push_str(t);
    };

    // Mutable borrows for closures work the same as if you were to create a normal mutable reference
    // for the scope in which the closure is used, we cannot create a nother reference to x;
    // e.g. if we remove the second closure invocation, would work since reference scopes now don't overlap
    closure("!");
    let z = &x;
    closure("!");

}

fn move_example() {

    let x = String::from("Hello, World!");

    // Move keyword moves captured variables into the closure
    // closure has ownerhip of x
    // this occurs regardless of how x is used
    // also the function trait that the closure implements is still Fn even though it owns x
    // this is because the function trait implementation still depends on the usage rather than the fact it has ownership

    let closure = move  || {
        println!("{x}");
    };

    // so this won't work since closure owns x;
    println!("{x}");
}
