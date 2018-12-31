/**
 * Rust offers the usual primitives: integers, floats, strings and booleans.
 * 
 * They also offer built-in _tuples_ for arbitrary small groups of data,
 * and an array, which is defined by *both* type and length (as in golang).
 * 
 * So: no surprises regarding primitive data types at first glance. 
 * 
 * The only unusual thing is that there is *no* concept of a `nil` or `null`
 * or `undefined` or `None`. All of those things are unsafe, and there is no
 * corresponding type for them. Variables can be temporarily unitialized in
 * your code flow, but the compiler will ensure that you never ever try to 
 * access them while they are in that state. Therefore, there is no need for
 * them to have a low-level language type of their own. 
 * 
 * The closest equivalent would be the `None` value of an `Option` enum.
 * That enum is well-known, fully encapsulated, part of the standard library, 
 * and even part of the *prelude* (so it is automatically imported and made
 * available in every Rust file). Thus the *concept* of `None` or `nil` does 
 * exist, as does the tool to deal with it. But it is not a primitive *type*. 
 * Dealing with that concept at a higher level of abstraction is *safer*
 * than having it running about as a primitive type, and that kind of safety
 * is what Rust is all about.
 */
fn main() {

    // as in most languages, strings come in two flavors, one primitive and
    // one higher level: `str` is the former and `String` is the latter

    // note that declaring a var of type `str` is wordier than you'd think!
    // The extra `&` is required, not optional, because technically this next
    // var is a __slice__ reference, aka a "reference to a string slice".
    let s: &str = "foo"; 
    // The compiler won't even let you declare `str` as a type... you will  
    // that you must always use `&str`: (reminder: is a "string slice". This 
    // will also be true when you declare types for string parameters and 
    // return values in function signatures. It's always `&str`, not `str`.

    // As a result, "simple strings" are not really so simple in Rust as
    // they are in other languages. Given the current state of unicode
    // multibyte madness, it's reasonable to stop considering strings as
    // something that should be simple. They're not simple at all!
    println!("I am a small-s string: {}", s);

    // most of the time we want our strings to be *expandable*, which means we
    // are really talking about a collection of char values. 
    // For that, rust gives us the the String class
    // Most other languages prefer their small-s string primitives, but not Rust!
    // in Rust we're more likely to use the big-S String 
    let big_s: String = String::from("bar");
    println!("I am a big-S String: {}", big_s);
    // NB: String args and return value types can appear in function signatures
    // as `&str`. So "string slice" is treated as a _superset_ of String.

    // Rust has a `char` type, which is the *only* valid place for single quotes
    let c: char = 'a';
    println!("I am a char: {}", c);

    // typed languages do love to get very persnickety about their numbers 
    let i_32 : i32 = 1_000_000; 
    println!("One MEEEELION dollars: ${}", i_32);

    // everyone uses float64 instead of float32, because it is just as fast, 
    // and less prone to forehead-slapping floating-point-math errors
    let f_64 : f64 = 3.1415927; 
    println!("Close enough to get to Pluto!: {}", f_64);

    // bool is either true or false
    // 0 is NOT the same as false, other numbers are NOT the same as true 
    // let oops : bool = 1; // compiler error
    let really : bool = true; 
    println!("Rust prefers truth over truthiness: {}", really);

    // no truthiness means no boolean shortcuts!
    // if 1 {  // compiler error
    //    println!("truthiness forever!");
    //}

    // there is no nil, None, null, or undefined 

    // run offers tuples, which are super-lightweight and dynamic-feeling
    // in the context of this strongly-typed language
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // println! won't do any auto-formatting of your tuple though
    // println!("I am a tuple, hear me roar: {}", tup); // compiler error
    println!("I am a tuple, hear me roar: ({}, {}, {})", tup.0, tup.1, tup.2);

    // finally, `arrays` are defined by both *type* and *length* (as in golang)
    let a: [i32; 5] = [11, 22, 33, 44, 55];
    let length = a.len(); 
    // there are more-idiomatic helper methods for this, I'm sure
    let last = a[length - 1];
    println!("I am an array of length {} ending in {}", length, last);

    // the fact that the `type` is pinned to a specific *length* makes rust 
    // arrays seem less practical than JS arrays or Python Lists. Most of the 
    // time you will probably prefer to use the non-primitive, higher-level 
    // Vector and Slice constructs instead of raw arrays. 

    println!("Rust has all the other usual suspects (maps, etc) in the standard library");
    println!("But none of them are _primitives_ per se")
}
