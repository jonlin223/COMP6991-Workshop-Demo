use week04::University;

fn main() {
    // DEFINITIONS
    // Crate -> smallest unit of compilation
        // Binary Crate -> main.rs: has a main function which compiles to an executable
        // Library Crate -> lib.rs: does not have a main function
    // Package
        // A Rust 'project', containing a Cargo.toml file

    // Honestly don't worry too much about the difference between the two
    // it's confusing since crates.io distributes packages...
    
    // Our Project Structure
    // week04 (package)
    // ├── Cargo.lock
    // ├── Cargo.toml
    // └── src
    //     ├── person
    //     │   └── mod.rs
    //     │   └── more_stuff.rs
    //     ├── faculty.rs
    //     └── main.rs <- cargo compiles starting here
    //     └── lib.rs  <- or here...

    // Modules
    // Oragnise code
        // readability
        // re use
        // privacy
    // Scopes
        // undefined -> private: can be accessed by current module and its children
        // pub -> can be accessed outside the current module
        // pub(super) -> can be accessed in parent module
        // pub(crate) -> can be accessed in the crate (e.g. for University)

    // Why does this not work?
    let unsw = University::new(vec![], vec![]);
    // unsw.print_faculties();

    // Documentation

    // Testing
    // tests module is standardised -> can put in each module to create unit tests
    // #[cfg(test)]
    // mod tests {
    // #[test]
    // fn function()
    // }
    // Also integration tests -> see rust book

}
