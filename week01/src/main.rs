fn main() {
    // Variables and Mutability
    let x = 10;
    let mut y = 10;
    let mut s = "Hello, World!";

    // How does Rust behave here?
    let f = y;
    y = 20;

    // Shadowing
    {
        let x = 100;
    }

    // Statements and Expressions
    let z = x + y;
    let a = {
        let x = 100;
        x + z
    };
    let b = return_value();

    let h = if true {
        // Stuff happens
        10
    } else {
        // other stuff happens
        20
    };

    // Options -> dividing by zero
    // What is the issue with null?
    Some(x);
    None


    // Control Flow
    // Match
    // If Let
    // Loops



}

fn return_value() -> i32 {
    100
}

fn add(x: u32, y: i32) -> i32 {
    x + y
}
