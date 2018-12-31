/**
 * We do not need to declare ourselves as a submodule!
 * Users will access us from the root as things::vegetable
 * 
 * So it's the `things` module which declares us.
 * 
 */

#[derive(Debug)]
pub struct Vegetable {
    pub name: String,
    id: String,
}

impl Vegetable {
    pub fn new(name: &str) -> Vegetable {
        Vegetable {
            name: String::from(name),
            id: super::get_id(8),
        }
    }
}
