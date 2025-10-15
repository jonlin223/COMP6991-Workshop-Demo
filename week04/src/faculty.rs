use crate::Actions;

#[derive(Debug)]
pub struct Faculty {
    name: String,
    office: String
}

mod something {
    struct Something {
        stuff: i32,
    }
}

impl Actions for Faculty {
    
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