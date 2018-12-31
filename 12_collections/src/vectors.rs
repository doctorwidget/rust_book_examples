/**
 * Vectors are a much closer analogue to Python's Lists or Javascript's arrays
 * than a Rust Array is. 
 */
pub fn demo_vectors() {
    let divider = "///////////";

    // Vectors are Rusts' growable generic collection
    // The library provides Vec<T>, where T can be any type you like
    
    println!("{}", &divider);
    println!("--- Vector Demonstration Begins --- ");

    // Instantiate a Vector from an array literal with the `vec!` macro:
    let v1 = vec![1, 3, 5, 7];
    println!("Vector from array literal: {:?}", &v1);
    // The above example uses Rust type inference. But if you declare an empty 
    // Vector and do not populate it on the same line, then the compiler cannot 
    // do type inference, and you might want to declare the type explicitly.
    let v2: Vec<i32> = Vec::new(); 
    // it is perfectly legit -- and not at all uncommon -- to start with an 
    // empty Vector, so this will probably come up pretty often in practice.
    println!("Empty vector (type i32): {:?}", &v2);

    // both of the above vectors are *immutable*. Don't be fooled by the idea
    // that Vectors are "growable".  A Vector is growable if it was declared
    // to be mutable, but immutable Vectors are the default. And they're still
    // totally useful that way, for their associated utility methods.

    // Here's a growable one:
    let mut v3 = Vec::new(); // type is presumably in a schroedinger state here
    v3.push(1);
    v3.push(3);
    v3.push(5);
    v3.push(7);
    
    // do Vectors support value equality out of the box?
    println!("Is v1 ({:?}) value equal to v3 ({:?}? {}", &v1, &v3, v1 == v3);

    // Now do a further mutation:
    v3.push(9);
    // and try it again
    println!("Is v1 ({:?}) value equal to v3 ({:?}? {}", &v1, &v3, v1 == v3);
    // (I am a little surprised again that the borrow checker does not complain here)

    // array-like access works
    let val1 = &v1[0]; 
    println!("First element of v3: {}", val1);
    // but runs the risk of panic! if you go out of range
    //let val2 = &v1[1001]; // runtime panic (not a compiler error!)
    // so use `get()`, which returns an Option<&T>
    let val3 = v1.get(1001); // an Option<&i32>, which will be None

    match val3 {
        Some(foo) => println!("WTF, how did we get {}", foo),
        None => println!("Yep, we expected None and that's just what we got")
    }

    // you can _make_ an immutable borrow reference without the compiler complaining
    let _valx = &v3[0]; // compiler doesn't complain about this
    // and you can still _push_ afterwards without the compiler complaining either
    v3.push(42);
    // but you can no longer *use* the _valx reference in the println! macro
    // without triggering a compiler error about crossed streams
    // println!("Why is the borrow checker letting us off so easily? {:?}", _valx);

    // tl;dr: the borrow checker is pretty lax about sequential declarations that
    // happen to cross the streams. It's only when you try to transfer one of the
    // declared references (even a `&` borrowed one!) to another scope (a function
    // or a macro) that it starts getting picky

    // iterate over a Vector with `for ... in ...`
    // but be sure to iterate over a _reference_ to the Vector unless you want
    // to transfer ownership (in which case the original will be gone, buh-bye!)
    print!("v3 has");
    for i in &v3 {
        print!("...{}", i);
    }
    println!("");
    println!("And it's still valid: {:?}", &v3);

    println!("--- Vector Demonstration Finish --- ");
    println!("{}", &divider)
}