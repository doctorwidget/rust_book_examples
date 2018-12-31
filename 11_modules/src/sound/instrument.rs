/**
 * A _submodule_ within the _sound_ module
 * 
 * Note that we don't declare `mod sound` or `mod instrument` anywhere in this
 * hierarchy! Both of those declarations did occur, but higher up:
 * 
 * - the root level .../src/main.rs file explicitly declared that it would try
 *   to use a `sound` module by declaring `mod sound;`, ending with a
 *   semicolon instead of a block {}.
 * - which led the compiler to the root level .../src/sound.rs file.
 *   That file in turn made an explicit `mod instrument;` declaration,
 *   again ending with a semicolon instead of braces.
 * - that let the compiler to the .../src/sound/instrument.rs, which is this!
 *   NB: the jump into a `sound/` directory still seems hand-wavy to me.
 * 
 * Then we are here, and anything we mark with `pub` is exportable. 
 * Everything _not_ marked with `pub` is private by default. 
 */

// without the `pub`, this function would be invisible to all other files. 
// With the `pub`, other files can `use` it, based on the directory path
// shenanigans described above. 
pub fn clarinet(path: &str) {
    println!("A clarinet goes tweet tweet! (via {} path)", path);
}
