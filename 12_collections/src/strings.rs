/**
 * Big-S strings: meeting all your string manipulation needs since 2009!
 * 
 * Note that there is no primitive "string" type per se... the closest you can 
 * ever get is `str`, which is a *string slice*, and not a "string". It is
 * usually used as a borrowed reference `&str` as opposed to as an ownership
 * variable. Because it's a *slice*, a string slice always has the standard 
 * start and end indices somewhere implicitly, even if they're not written out.
 * Thus, there is implicit magic here, which always makes me uncomfortable. 
 * 
 * If it helps, consider that the alternative would be to deal directly with 
 * arrays of type `u8`. The underlying implementation for big-S strings is in
 * fact a Vec<u8. But nobody wants to deal with _that_. So "string slices"
 * plus Big-S Strings are what we've got instead.
 * 
 * And te big-S String is exactly as it appears to be: a classic collection
 * class, with utility methods galore. Like a Vector, it is *growable*, which
 * is exactly what you expect from a modern string manipulation class. 
 * 
 * The compiler makes it easy on you by auto-casting fairly aggressively 
 * between big-S strings and `&str` references. That kind of auto-casting is
 * inherently dangerous and seems somewhat in contrast to the general Rust
 * philosophy, tbh. But if you're going to have compiler automagic behavior
 * anywhere, implementing it for something as fundamental as strings seems
 * like a pretty defensible choice. 
 * 
 */
// for randomization
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric; // an Enum variant, presumably
// for unicode
use unicode_segmentation::UnicodeSegmentation as Uni;

// get random strings -- made public so others can use it also
pub fn rand_str(length: usize) -> String {
  let rand_string: String = thread_rng()  // rng for `Random Number Generator`
        .sample_iter(&Alphanumeric)   // distributions enum, Alphanumeric variant
        .take(length)  // take however many you like
        .collect();    // and bundle them up together into a String
  // ending with rand_string all by itself as an _expression_ has the same
  // effect as saying `return rand_string;`, but it is more idiomatic in Rust
  rand_string
}

// demo string-related code 
pub fn demo_strings () {
    let divider = "///////////";

    // Strings are Rusts' growable character collection
    
    println!("{}", &divider);
    println!("--- String Demonstration Begins --- ");

    // the data _variable_ is a "string slice", but we have initialized
    // it with a string *literal*. A string literal is not a variable!
    let data = "foobar"; 
    // string slice variables get initialized by string literals,
    // but they don't have a *type* of "string literal";
    // they have a type of "string slice".
    
    // here are two ways to go from string literals to big-S strings
    let str1 = "foobar".to_string(); // that's to-big-S-string, really
    let str2 = String::from("foobar");
    // both of those references are to the type `String`, not some weird
    // string slice thing. So they don't have a string literal type either.

    let str3 = data.to_string(); // but you can call string literal methods on string slices
    let str4 = String::from(data); // and feed them to String::from as arguments
    // so this may be a distinction without a difference,
    // given how aggressively the compiler auto-casts everything.
    // Still, I am wary of this issue... /suspicious

    println!("Equal big-S strings? ({}, {}): {}", &str1, &str2, &str1 == &str2);
    println!("Second big-S pair is also equal: {}", str3 == str4);
    println!("Big-S equals string slice? ({}, {}): {}", &str1, &data, &str1 == &data);

    let rand1 = rand_str(11);
    println!("A random string: {}", &rand1);

    // Just as we saw with Vec<T>, a big-S String *can* be made mutable, 
    // and has a bunch of methods that *support* mutability,
    // but they are still immutable by *default*
    // rand1.push_str(" -- apple"); // compiler error: cannot borrow rand1 as mutable!

    // you have to declare a String as mutable right from the start for that to work
    let mut mut1 = String::from("foo");
    mut1.push_str(" -- "); // can push a string literal directly
    mut1.push_str(&rand1); // but must explicitly borrow a big-S string
    // nb: you could use .push() if the argument was a single char
    println!("A mutable string: {}", &mut1);
    // note the compiler automagic: we can use {} instead of {:?} for big-s Strings

    // the '+' operator is overloaded for big-S string addition, 
    // but with some rather specific limitations: 
    // first, note that we're not using any of these strings as mutable references.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // *and* note that the first reference is an ownership reference,
    // whereas all the other big-S references must be borrows, 
    // and the string literals just slide on in there 
    let s4 = s1 + "-" + &s2 + "-" + &s3;
    println!("+ operator overloading: {}", &s4);

    // Alternatively, you can use the `format!` macro, which requires fewer
    // ownership shenanigans
    let r1 = String::from("55");
    let g1 = String::from("155");
    let b1 = String::from("200");
    let rgb = format!("rgb({}, {}, {})", r1, g1, b1);
    // all values are still owned by their original owners: 
    println!("All values remain owned: {}, {}, {}, {}", r1, g1, b1, rgb);

    // NB: you cannot use array-notation access on strings (big-S or otherwise),
    // because it's the 21st century, and everything is unicode, baby!
    // A big-S String is a Vec<u8>, but one on-screen glyph only corresponds
    // to one u8 value (aka one _byte_) if you're still working in ASCII. 
    // In Unicode, one glyph can be 1, 2, 3 or 4 bytes. 
    // Hence weirdness like this:
    let hola = String::from("hola"); // 4 glyphs, 4 bytes
    let russ = String::from("Здравствуйте"); // 12 glyphs, but how many bytes?
    println!("{} is {} bytes long and it has 4 glyphs", hola, hola.len());
    println!("But {} is {} bytes long, though it has 12 glyphs", russ, russ.len());
    // the rust book refers to a glyph as a "grapheme cluster"
    // that's a terrible choice for a term of art, even if it's in the specs
    // "Glyph" or "rune" (as in golang)) are both much better terms for this.

    // the de-facto standard for dealing with glyphs is "unicode-segmentation"
    // https://crates.io/crates/unicode-segmentation
    // we added that to the root `Cargo.toml` for the project,
    // and we did a `use` declaration for it above, so now let's use it!
    // Note that in this case the compiler auto-cast fails us,
    // and we have to explicitly convert from big-S string to string slice
    // (that's the `russ.as_str()` in the next line)
    let decoded = Uni::graphemes(russ.as_str(), true).collect::<Vec<&str>>();
    // also note the rather-ugly type `<Vec<&str>>`... yikes!
    // but this does what we want
    println!("Unicode-Decoded Russ is now: {:?}", decoded);
    println!("And we can *objectively* say that Russ has {} glyphs", decoded.len());
    println!("Even though the original unicode has {} bytes", russ.len());

    // back to built-in methods:
    // you can call .chars() on a string slice to get the individual bytes (u8 values),
    // and then do a `for ... in ...` over them 
    let kanji = String::from("नमस्ते");
    println!("Using .chars() on kanji to get the individual bytes");
    for c in kanji.chars() {
      print!("...{}", c);
    }
    println!("... (done)");
    // but again, that's not actually the recommended way to deal with unicode!
    // So that's a six-byte unicode string, but it has only four characters
    let decoded_k = Uni::graphemes(kanji.as_str(), true).collect::<Vec<&str>>();
    println!("Unicode-Decoded Kanji is now: {:?}", decoded_k);
    println!("So now we see that the kanji var has {} glyphs", decoded_k.len());
    println!("Even though the original unicode has a length of {}", kanji.len());
    // note the 100% inconsistent API here: need to use .count() on result of .chars()
    println!("And the original unicode has {} chars", kanji.chars().count());
    // but you can use .len() on result of .bytes()
    println!("And the original unicode has {} bytes", kanji.bytes().len());

    println!("--- String Demonstration Finish --- ");
    println!("{}", &divider)
}
