/**
 * Rust tools: 
 * 
 * - rustc is the compiler 
 * - rustfmt is the auto-formatter 
 * - cargo is your build tool and project manager, like lein for Clojure
 * - rustup is the highest-level meta tool
 * 
 * As mentioned, {rustup} is the meta tool: you use this to make sure you
 * are using the latest versions of all your rust tools. 
 * 
 *  $ rustup update stable
 * 
 * And that will bring all the other rust tools up-to-date. Remember that
 * the initial installation of the entire rust ecosystem began with: 
 *
 *  $ curl https://sh.rustup.rs -sSf | sh
 * 
 * Once you have access to {cargo}, you start brand new projects with it via:  
 *  
 *  $ cargo new foo
 * 
 * That will create a new foo/ directory, which you should then cd into.
 * 
 * From inside the foo/ directory, you can do a one-shot compile & run via: 
 * 
 *   $ cargo run 
 * 
 * Which is _different_ from a full build! A full build creates an exe file 
 * (or whatever the equivalent is for the current OS): 
 * 
 *   $ cargo build 
 * 
 * You can also simply _check_ to see if there are any compiler errors: 
 * 
 *   $ cargo check  
 * 
 * And once you've written some automated tests, you can run them via: 
 * 
 *   $ cargo test 
 * 
 */
fn main() {
    println!("Hello, world!");
}
