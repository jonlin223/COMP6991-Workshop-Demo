use crate::{faculty::Faculty, person::Person};

mod faculty;
mod person;
mod location;

pub struct University {
    faculties: Vec<Faculty>, // which we can shorten to...
    people: Vec<Person>
}

impl University {
    ///
    /// # 
    pub fn print_faculties(&self) {
        println!("{:?}", self.faculties);
    }

    /// # Parameters
    /// faculty: Vec
    /// People: vec
    pub fn new(faculty: Vec<Faculty>, people: Vec<Person>) -> Self {
        University { faculties: faculty, people: people }
    }
}


pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use crate::add_one;

    #[test]
    fn test_add_one() {
        let x = 1;
        assert_eq!(add_one(x), 2);
    }
}

trait Actions {
    fn say_name(&self) {
        println!("HELLO?");
    }
}
