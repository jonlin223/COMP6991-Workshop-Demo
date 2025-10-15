use crate::Actions;

struct Location {
    name: String,
}

impl Actions for Location {
    fn say_name(&self) {
        println!("The name of the location is {}", self.name);
    }
}