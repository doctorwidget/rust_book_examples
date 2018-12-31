/**
 * Testing has first-class support in Rust, hooray!
 * 
 * There are numerous by-convention features baked into Rust (and Cargo) that
 * make testing convenient and simple. 
 * 
 * Test functions should be annotated with the `#[test]` annotation, and
 * if the entire module is intended to be used only for testing, then the
 * module itself should be annotated with `#[cfg(test)].
 *  
 * Functions and modules with those annotations will *only* be compiled and 
 * run when you execute `cargo test`, and are omitted from binary builds. 
 * Those annotations acts as both an  *only* and an *always*: they make sure 
 * that all your tests always and only run on every `cargo test`.
 * 
 * All files placed in a `{root}/tests/` will be treated as integration
 * tests. They get run only when executing `cargo test`, and they are never
 * added to final binaries. In effect, they have a `#[cfg](test)] annotation
 * automagically applied to their entire file, by dint of living inside the
 * `{root}/tests/` directory.
 * 
 * SO: go look inside {root}/tests/ for separated-out tests, AND also in 
 * the lib.rs file for inlined tests. 
 * 
 * (A Note On Libraries & Binaries )
 * 
 * Note that this module includes both a single binary and a single library.
 * 
 * The single binary is this `main.rs` file. Any `src/main.rs` file is the
 * singleton binary by default: you don't have to explicitly tell cargo to 
 * treat it that way. You have the *option* to configure your project to 
 * have multiple binaries if desired, but that is optional.
 * 
 * The library file is named `lib.rs` by convention. It must be *configured*
 * in the main project cargo file: `{root}/Cargo.toml`. See that file for 
 * the details of configuring a library file. 
 * 
 * There can only ever be zero or one libraries in a project. Contrast that 
 * with binaries, where there can be zero to N binaries. There must always 
 * be at least one binary OR one library: a project without at least one of 
 * the two is not valid. When you execute `cargo new foo`, it gives you a 
 * binary setup by default, but there are flags to start with a library-only
 * project instead if you like.  
 * 
 */
use::mylib;

fn main() {
    mylib::echo("Hello Library");
}
