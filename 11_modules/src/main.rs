/**
 * Rust's module/package system is *very* fully-featured and rich.
 * It's worth revisiting the rust book chapter, which is chock full of
 * special-case details, synonyms, tricks and tips. 
 * 
 *   https://doc.rust-lang.org/book/ch07-00-packages-crates-and-modules.html 
 * 
 * 
 * Here are a few of the highest-level details before we start:
 * 
 * - Cargo expects 0 or 1 `src/main.rs` per project (i.e. per Cargo.toml)
 * - Cargo expects 0 or 1 `src/lib.rs` per project (i.e. per Cargo.toml)
 * - The `main.js` becomes an executable / binary
 * - The `lib.js` becomes an exportable library
 * - Additional top-level executable files can go into `src/bin/foo.rs`
 *      Each such `*.rs` file will become one (1) top-level binary
 * - Other top-level files in `src/` are importable within the project,
 *      but do not become exportable automatically.
 * - You can combine keywords in your lib.rs to re-export everything, 
 *      but you only get that one anointed lib.rs file for that purpose.
 * 
 * Until now we have never used the `mod` keyword, which has made it easy
 * to do simple `hello world` style examples. That has meant that all the
 * code in each demo file is in the same namespace. Everything is visible
 * to everything else, which has made everything easy, hooray! But at the
 * same time, *nothing* has been exportable outside of the single main.rs
 * files, which means *none* of that code can ever be re-used, booo!
 * 
 * The `mod` keyword is thus a double edged sword. As soon as you start 
 * using it:
 * 
 * - you suddenly have to deal with public vs private and _access_ issues 
 * - entities can now be *re-used* by other files that import the module 
 * 
 * Obviously that's a necessary trade-off for anything non-trivial, so let's
 * roll up our sleeves and get familiar with it!
 * 
 * This file is our one (1) `main.rs` for the project, so it will be our
 * one (1) executable.  However, we will make reference to four (4) other 
 * _modules_ for this project, with differing access strategies.
 * 
 * - `foo`, a module defined right here inline with this file
 *      But defining modules within `main.rs` is too trivial to be useful. 
 * - `spam`, a module defined in a sibling file (./spam.rs) all by itself
 *      This pattern is probably all you would need for small projects.
 * - `sounds`, a module defined in a sibling file (./sounds.rs) but which
 *      also has with associated subdirectories. This example comes from the
 *      official Rust book, but I don't like the dual use of a sounds.rs file
 *      with a ./sounds/ directory. There is implicit magic here which I dislike.
  * - `things`, a module defined in a sibling directory with an internal `mod.rs`
 *      file. This pattern comes from the Blandy & Orendorff book, and I like
 *      the fact that everything about it is explicit.  This is the one I would
 *      use in my own projects, but you have to be familiar with all of them!
 * 
 * Finally, we'll show the use of completely external modules inside `things`.
 * It uses the *external* crate `rand`, the de facto standard way to generate
 * random values. This library is not part of the rust core, but it *was* a
 * part of the core long ago, and it is still maintained by the same devs who
 * *do* maintain the rust core. So it's as anointed as you can get without 
 * being bundled with standard rust. 
 * 
 * All of those target modules are *somewhere* in the project, but none of them 
 * are part of the default public scope of this `main.rs` file. Therefore, we 
 * have to announce that we will be using each of them, and then provide the 
 * correct implementation target for each. Both steps are necessary! You must
 * _declare_ that you're going to use a module, and then you must _implement_
 * that module. The declarations take the same shape for all four modules, 
 * but their implementations are all different. 
 * 
 */

// For inline examples *only*, the `declaration` and the `implementation`
// take place in the same location. That's what the word "inline" means!
mod foo {
    // you are free to define as many nested submodules as you like
    // but remember that *everything* is private by default!
    // So we have to explicitly declare the submodules as public
    pub mod bar {
        // but eventually you will need a leaf node or why bother?
        pub fn zug(path: &str) {
            println!("I am Zug; hear me roar (via a {} path!)", path);
        }

        // this function demonstrates the use of `super::` 
        // this is the only way to reach *up* and *over*
        pub fn qux() {
            // without super::, qux cannot see up to blort
            // blort("qux cannot see blort directly"); 
            // compiler error > blort not found in this scope
            
            // but this works:
            super::blort("message from qux");
        }
    }

    // this fn is part of foo, so it c
    pub fn blort(msg: &str)  {
        println!("Blort says: {}", msg);
    }
}

// next, the series of *declarations*, each of which points to one of the
// module implementations discussed above. The declaration phase is easy to
// forget, because the implementations are all part of the project, and so 
// their source files are very close by. But you are *required* to make an
// explicit declaration nontheless. If you throw in references to `crate::x::y` 
// without having preceded them with one of these declarations (e.g. `mod x;`), 
// the compiler will error out with a "failed to resolve" error message.  

// (So remember!!): even though these modules are local to the project, you 
// cannot make *undeclared* relative or absolute path references to them!!

// Declare that we are looking for a `spam` module as a peer of some kind
// Note that this differs from `use`, which would mean we were expecting 
// Cargo to find the installed library in whatever cache directory it uses. 
// NB: this means that a peer/sibling directory is _not_ automatically treated
// as a module by default! Only peers that you declare in this way are modules.
mod spam; // treat a spam peer (of some kind!) as a module 
// in this case, it's a file: spam.rs
// and that file is self-contained, with no further path-based shenanigans

// Declare that we are looking for a `sound` module as a peer of some kind.
mod sound;  // treat a sound peer (of some kind!) as a module 
// ending in semicolon instead of braces tells the compiler to find this module
// In this case it is a `./sound.rs` file, which *happens* to include its own
// submodule in its own subdirectory. The peer file is *definitive*, but nested
// subdirectories are a *maybe*. I don't like maybe! Therefore, this pattern 
// bothers me, and I much prefer the next and final one, in which we have a
// top-level directory matching the module name, plus an explicit barrel file. 

// Declare that we are looking for a `things` module as a peer or some kind
mod things; // treat a things peer (of some kind!) as a module.
// in this case, it's a directory, which has an inner ./mod.js file, which 
// acts as the one-and-only barrel file for that module. I like this approach 
// _much_ better than the weird one used for sound, above.  Everything here is 
// explicit, and there is no compiler magic going on anywhere. 


fn main() {
    // module `foo` is the first and simplest example, since it is inline.
    // we can get to the inline `foo` module two ways:
    // Via an absolute path, starting with the language-level keyword `crate` 
    crate::foo::bar::zug("absolute");
    // Or via *relative* path, where anything that is a peer of `main`
    // can be used as the top of the path.
    foo::bar::zug("relative");

    // NB: you could also start with `super::` to back up one level
    // There is no need for a `sub::`, because that's what you're doing with `::`!
    foo::blort("message from main"); // call blort directly
    foo::bar::qux(); // qux also calls blort, via super:: shenanigans

    // module `spam` is the second-simplest example. It points to an all-in-one 
    // peer file whose contents are eerily similar to our inline foo, above.
    crate::spam::eggs::toast("absolute");
    spam::eggs::toast("relative");
    // and here is the same super:: demo we did with foo, but for spam
    spam::beans("message from main");
    spam::eggs::ham();


    // Then the `sound` module uses a weird pattern where there is 
    // both a `./sound.rs` peer file, and a `./sound/` peer directory.
    // The weirdest part is that the `sound.rs` peer file is allowed to
    // refer to the `instrument` file without specifying the true path:
    // there's just an implicit automagic compiler leap where it knows to
    // look for a ./sound/instrument.rs` file. This bothers me a lot!
    crate::sound::instrument::clarinet("absolute");
    // But once you get path that irritant, you can do the same absolute vs 
    // relative thing that we've demonstrated for everytone else. 
    sound::instrument::clarinet("relative");

    // module `things` shows a more-scalable approach to modules
    // There is no `things.rs`, but there *is* a ./things/ peer directory
    // and that directory has a `mod.rs` file, which acts as the top level
    // file for the module, much like `index.js` does in a node project.
    crate::things::greet();
    // use things via relative path
    let stuff = things::assortment();
    println!("An assortment of things: {:?}", stuff);

    // accessing nested modules can get verbose!
    let dog = things::animal::Animal::new("Rover");
    println!("Rover says 'ruff ruff': {:?}", dog);

    // use the `use` keyword to allow terser access
    use crate::things::mineral::Mineral; // the final segment is now in scope as is
    let coal = Mineral::new("Coal, ick!");
    println!("Hi! I cause global warming!: {:?}", coal);

    // the `as` option allows you to avoid namespace collisions if necessary
    use crate::things::vegetable::Vegetable as Plant;
    let oak = Plant::new("oak");
    println!("From a tiny acorn did I grow: {:?}", oak);
}

// there are still plenty of other little details to review in the article
// in the main Rust book. This is a big topic, because it is _important_! 
// For example, you can `pub use` to re-export under shorter names, and there
// are import syntaxes to condense multiple imports from sub-branches of the
// same overall module. And there's a wildcard glob '*' to import everything
// from a module, complete with the usual warnings about how that can be a bad
// thing, because it makes it much harder to trace relationships. 
// TODO: go back and re-read the whole chapter, seriously!
