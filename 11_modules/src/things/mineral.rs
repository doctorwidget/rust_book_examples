/**
 * Minerals submodule 
 */

#[derive(Debug)]
pub struct Mineral {
    pub name: String,
    id: String,
}

impl Mineral {
    pub fn new(name: &str) -> Mineral {
        Mineral {
            name: String::from(name),
            id: super::get_id(8),
        }
    }
}
