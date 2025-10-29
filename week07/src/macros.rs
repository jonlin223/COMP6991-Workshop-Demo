// What is a macro?
// Two types of macros
// Declarative macro... examples?
// Procedural macro... examples?

// You only need to know how to write declarative macros

// Macros "produce" code
// Are expanded before compilation (pre-compilation step)

// Why macros?
// Why can't we just use a function

#[derive(Debug)]
struct Example {}

// BASIC EXAMPLE
macro_rules! sum {
    ($x:expr, $y:expr) => {
        $x + $y
    };
}

fn example() {
    let z = sum!(1, 2);
}

// Metavariables
// expr -> an expression
// block -> { block }
// ident -> variable names
// ty -> type
// tt -> token tree

// Pattern matching

// REPETITION EXAMPLE
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
            )*
            v
        }
    };
}

fn example2() {
    let v = my_vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
}


// RECURSION EXAMPLE
macro_rules! cvar {
    () => {};

    ( ($type:ty) $name:ident = $value:expr; $( $rest:tt )* ) => {
        let $name: $type = $value;
        cvar!($($rest)*);
    };

    ( ($type:ty) mut $name:ident = $value:expr; $( $rest:tt )* ) => {
        let mut $name: $type = $value;
        cvar!($($rest)*);
    }
}

fn example3() {
    cvar! {
        (i32) foo = 42;
        (&str) mut bar = "hello";
        (char)  baz = 'z';
    }
}


