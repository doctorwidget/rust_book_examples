/**
 * Slices are references to _subsets_ of *collections*. Whenever you have any
 * sequential collection -- whether a true array or merely array-like, such as
 * a String or Vector -- you can create and manipulate *slice references* to  
 * your chosen subsets of that collection. 
 * 
 * Slices are a data type all their own, with some special rules for how they
 * are defined and manipulated. Thus they were a bit too complex to include
 * in the discussion of simple data types. At the same time, they are best
 * understood as a language primitive, and not some kind of higher-level class
 * from the standard library (e.g. like a Vector or the ubiquitous Option<T>).
 * 
 * A slice is a __compound__ data type, just like an array. Where an array is
 * defined by its type and its length, a slice is defined by a collection, a
 * start index, and an end index. Take away any of those three and it's not a
 * valid slice any more. 
 * 
 * Hence this is the general form:
 *        
 *       &foo[2..5]
 * 
 * Where `foo` is a collection (which must have been defined before this line!),
 * and the two indices live inside the square brackets. 
 */

fn main() {
    // most of our examples will be with a big-S String. 
    // This is a classic example of an array-like collection.
    let s = String::from("The quick brown fox");
    println!("Various examples based on 's': {}", s);

    // next, we take and print a variety of slices from it 
    // As always, `&` should be read as "a read-only reference"
    let quick = &s[4..9];
    println!("Second word via '&s[4..9]': {}", quick);
    // As usual, the start index is the first to be *included*
    // and the last index is the first to be *excluded*

    // you can *include* the final index by adding an `=`
    let quick2 = &s[4..=8];
    println!("Second word, via '&s[4..=8]': {}", quick2);
    // This looks awkward at first, but it's better than the alternative of
    // fiddling with a `+ 1` or `-1` on those occasions when you need it. 

    // as in Python, leaving off the first index is synonymous with 0
    let the = &s[..3];
    let other_the = &s[0..3];
    let both_the = [the, other_the];
    println!("Selected start via '&s[..3]' vs '&s[0..3]': {:?}", both_the);

    // and leaving off the final index is synonymous with 'to the end'
    let fox = &s[16..19]; // this way you have go *past* the length, yuck!
    let fox2 = &s[16..]; // this way you don't even need to know the length
    let both_foxes = [fox, fox2];
    println!("Selected end via '&s[16..19]' vs '&s[16..]': {:?}", both_foxes);

    // which means you can slice the whole thing super easily, 
    // without resorting to a clumsy reference to `.length` at the end!
    let dupe = &s[..]; // slice of the whole thing, hooray
    println!("Slice of the whole thing via '[..]': {:?}", dupe);

}
