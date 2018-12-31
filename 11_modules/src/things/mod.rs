/**
 * Barrel file for the things module. Users of the `things` module (the `things`)
 * only need to target _this_ directory. The name `mod.js` is the equivalent
 * of an `index.js` in JS: it declares this file to be the default file for
 * the directory, creating a one:one relationship between the directory and
 * this file, under the name of the directory. 
 * 
 * The first imports in this file will be from the external `rand` crate. 
 * We will use those functions down the road to generate uuid-like identifiers.
 * The general sequence for using an *external* library is as follows:
 * 
 * 1) add it to the [dependencies] section of the root `Cargo.toml` file
 * 2) _declare_ which specific aspects of the crate you will be using
 * 3) use it!
 * 
 * We can't show the first part in this file (duh). See `{root}/Cargo.toml`
 * for that. The other two both happen here.
 * 
 * (You will sometimes see older docs (& posts & etc) that use an additional
 * step of the form `extern crate foo`. That was required with the original
 * `rustc` compiler, before the `cargo` build tool became standard. As of 
 * 2018, and assuming you are using `cargo`, you will never need to specify
 * `extern crate foo`)
 * 
 * After the rand-related imports, we define some sub-modules for this one.  
 * All three of these references end in a semicolon instead of a block {}. 
 * Therefore, the compiler will look for all of them as  _peer files_. We are 
 * defining these as modules from *above*: when you look into these files 
 * themselves, they will make no mention of their own module names. In other
 * words, each of these files "thinks" of itself as its own root. In Rust,
 * *modularity is in the eye of the beholder*. 
 */
// note the shortcut form for >1 import from a library: {thread_rng, Rng}
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric; // an Enum variant, presumably

pub mod animal;
pub mod mineral;
pub mod vegetable;

#[derive(Debug)]
pub enum Thing {
  Ani(animal::Animal),
  Min(mineral::Mineral),
  Veg(vegetable::Vegetable),
}

pub fn assortment() -> [Thing; 3] {
  let a = Thing::Ani(animal::Animal::new("Cat"));
  let m = Thing::Min(mineral::Mineral::new("Topaz"));
  let v = Thing::Veg(vegetable::Vegetable::new("Rose"));
  return [a, m, v];
}

pub fn greet() {
  println!("Greetings from the things module ({})", &get_id(7));
}

// Get a uuid-like pseudorandom identifier
// Note that this function is *not* flagged as `pub`. That means whoever imports
// this module cannot call this function, because access is *private* by default 
// whenever an entity is looking *downwards*, deeper into a module or submodule. 
// But the submodules of this module *can* call it, because access is *public*
// by default when a submodule looks *up* into its own module ancestry tree.
fn get_id(length: usize) -> String {
  let rand_string: String = thread_rng()  // rng for `Random Number Generator`
        .sample_iter(&Alphanumeric)   // distributions enum, Alphanumeric variant
        .take(length)  // take however many you like
        .collect();    // and bundle them up together into a String
  // ending with rand_string all by itself as an _expression_ has the same
  // effect as saying `return rand_string;`, but it is more idiomatic in Rust
  rand_string
}
