use std::vec;

use crate::{faculty::Faculty, Actions, University};

pub struct Person {
    name: String,
    zid: String,
    role: PersonRole
}

enum PersonRole {
    Staff,
    Student
}

fn parent_from_child() {
    let u = University { faculties: vec![], people: vec![] };
}

impl Actions for Person {
    fn say_name(&self) {
        println!("My name is {}", self.name);
    }
}

mod more_stuff;