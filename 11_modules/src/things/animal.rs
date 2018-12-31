/**
 * Animals submodule 
 */
#[derive(Debug)]
pub struct Animal {
    pub name: String,
    id: String,
}

impl Animal {
    pub fn new(name: &str) -> Animal {
        Animal {
            name: String::from(name),
            // note the use of `super` to reach up one tier in the hierarchy
            id: super::get_id(8),
        }
    }
}

