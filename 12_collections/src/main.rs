/**
 * There are a few collection types that are used so ubiquitously (across all
 * languages, not just Rust!) that they have been made part of the standard
 * library. Two of them (String and Vector) are even part of the _prelude_, 
 * so you don't even need to do any import shenanigans to use them in any 
 * particular file. The third -- HashMap -- is not part of the prelude, though
 * *I* would have put it there if anyone had asked me! So none of those types
 * are part of the lanuage core, but they are all readily available.
 * 
 * We'll do some quick demos of all three types here
 */
mod vectors;
mod strings;
mod hashmaps;

fn main() {

    // use Vec<T> for arbitrary growable collections of any type T
    vectors::demo_vectors();

    // use Strings for collections of characters
    strings::demo_strings();

    // use HashMaps for... just about everything!
    hashmaps::demo_hashmaps();
}
