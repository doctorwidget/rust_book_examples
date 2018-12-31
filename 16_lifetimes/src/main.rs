/**
 * Lifetimes are the *second* big innovation that makes Rust unique.
 * We saw the first one long ago: ownership (THERE CAN BE ONLY ONE).
 * 
 * Lifetimes refer to the scope for which reference is valid. When a variable
 * is out of scope, the lifetime of that variable is over, and the compiler 
 * is free to `drop` that variable at its earliest convenience. Sometimes the 
 * lifetime of a reference is obvious - the nearest enclosing set of curly 
 * brackets works 99% of the time for trivial demo programs, the likes of 
 * which we've been writing until now. But things get trickier as soon as you 
 * send references *into* or *out* of functions or expressions. At that point 
 * a simple reading of the nearest curly brackets is no longer definitive. In
 * other words, it is possible for references to *escape* their current block, 
 * and you must be prepared to deal with the consequences (dun dun dun...).
 * 
 * In a garbage-collected language, the garbage collector is constantly aware
 * of this issue, but it never makes the programmer think about it. In Rust, 
 * you can *often* ignore this issue -- look how far we've come before even 
 * mentioning it! But you cannot *always* ignore it. And there may be times 
 * where you absolutely *want* to deal with it proactively, on your own terms,
 * rather than accepting the default values that the compiler infers for you. 
 * 
 * Explicit lifetime values are _generic_. Your explicit lifetime annotations
 * never *change* the lifetime of any individual variable: that is still 
 * decided objectively by the compiler, based on exhaustive analysis of all 
 * code paths. All your lifetime annotations ever do is *clarify* the generic
 * relationships between variable lifetimes, in cases where there is more than
 * one variable, and their lifetimes are ambiguous. This is the *only* purpose
 * of explicit lifetime annotations! And that's important enough to repeat: 
 * 
 *      Explicit lifetime annotations clarify things for the compiler!
 * 
 * I'd put that in all caps, but it's not as catchy as THERE CAN BE ONLY ONE.
 * 
 * Lifetime annotations are of the form 'a, where the `'` is required and the
 * letter is a single lowercase character by convention. Think of `'a` for 
 * lifetime annotations as being analagous to the `<T>` used for Generics. 
 * Just as with Generics, you'll need one letter per lifetime _type_. It is
 * idiomatic to start with 'a and possibly 'b, and then a 'c if needed, etc. 
 * 
 * You *only* need these when there are 2 or more vars whose lifetimes are
 * ambiguous. But that does *not* mean you always need both `'a` and `'b`,
 * because sometimes all you need to do is use `'a` twice, to clarify to the
 * compiler that vars x and y are expected to share the same lifetime `'a`.  
 * 
 * Lifetime annotations can appear in several places. One place is inside the 
 * generics clause -- right there in the same angle brackets used by generics! 
 * 
 *      fn foo<'a>(...) {...}
 * 
 * That does not mean we need to invent a new name for "the generics clause",
 * because lifetime annotations are best understood as a *type* of generic. 
 * Hence the generics clause is exactly where we expect them to live, tyvm!
 * 
 * The other place they appear is as part of any variable type declaration:
 * 
 *      let x: &'a i32;
 *      
 *      fn foo<'a>(y: &'a i32, z: &'a mut i32) {...}
 *
 * It should be clear that these are a _fundamental_ language feature, even if
 * you don't have to use them very often. Expect them to be much more common
 * in useful real-world code than you'd think from looking at all prior demos.  
 * 
 * Appendix: A Few More Terms of Art Regarding Rust Lifetimes
 * 
 * - lifetime elision rules:  patterns used by the compiler to infer lifetimes
 * - input lifetimes: lifetime annotations added to input arguments
 * - output lifetimes: lifetime annotations added to return values
 * 
 * The Rust team reserves the right to add more lifetime elision rules in the 
 * future. So you might need to specify _fewer_ explicit lifetime annotations
 * in the future, because the compiler might get smarter, and consider fewer
 * scenarios to be ambiguous. It's very unlikely that the compiler will ever  
 * become less sophisticated and require more manual annotations.
 */

fn simple_scope () {
    // demo of simplest possible lifetime issues
    let _r0; // this r is a reference to an integer   
    // it's `mut` only because we're going to assign to it twice         
    
    // arbitrary block just for demonstration of lifetimes
    {                     
        let x = 5; // x lifetime is the enclosing block by default
        _r0 = &x;           
        // r is *not* a reference to a primitive '5'
        // r *is* a reference to the x _variable_!
    } // lifetime of x is over now
    // compiler calls `drop x`, and x is gone, buh-bye!     

    // compiler error: `x` does not live long enough
    // println!("_r0 is: {}", _r0); // cannot compile with this uncommented

    // but the same pattern with no block works fine
    let r1;
    let y = 7;
    r1 = &y;
    println!("r1 is: {}", r1); // y is still alive, so all is well

    // one way around this would be use only _ownership_ references
    // (but this is, of course, limiting in many scenarios)
    let r2; // r2 must *own* its integer; it cannot borrow it!
    {
        let z = 11; 
        r2 = z; // ownership transfer (THERE CAN BE ONLY ONE) 
        // this time r points to the primitive 5
    } // lifetime of y is over now
    // compiler calls `drop y` and y is gone, buh-bye!
    println!("r2 is: {}", r2);

    // or with even fewer scope issues
}

fn explicit_lifetime() {
    // let's show explicit lifetimes in action...HOW EXCITING
    // a big-S String... heavyweight, growable, etc
    let string1 = String::from("abcd");
    println!("string1 is: '{}'", string1);
    // a string slice... closest we can get to a string primitive
    let string2 = "xyz";
    println!("string2 is: '{}'", string2);

    // failed helper function - does not even _compile_ because of lifetime issues!
    //fn fail_longest(x: &str, y: &str) -> &str {
    //    if x.len() > y.len() {
    //        x
    //    } else {
    //        y
    //    }
    //}    
    // compiler error is: `expected lifetime parameter`
    // This will return a ref to _either_ x *or* y, but we cannot say which!
    // This signature does *not* say x and y must have the same lifetime
    // which means they could come in with very *different* lifetimes
    // So the compiler *cannot* safely infer the correct lifetime.
    // Which means we must specify it.
    // Hence the fn above will *never* compile, and we have to comment it out

    // if it could compile, here's how we would use it
    // let result = fail_longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // So let's write a lifetime-aware function!
    // As mentioned at the top of this file, this is only to clarify lifetimes
    // for the compiler, since we know they are ambiguous. 
    fn naive_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        // the signature above uses `'a` _four_ (4!) times to clarify lifetimes
        // That's four references to 'a to tell the compiler that the three (3)
        // references must all share the _same_ lifetime.
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let result = naive_longest(string1.as_str(), string2);
    println!("(naive) The longest string is '{}'", result);

    // so why are we calling this 'naive'? It seems to work fine!
    // Here's another example of it working fine:
    let string3 = String::from("long string is long");
    println!("string3 is: '{}'", string3);
    {
        let string4 = String::from("xyz");
        println!("string4 is: '{}'", string4);
        let result2 = naive_longest(string3.as_str(), string4.as_str());
        println!("(naive) The longest string is '{}'", result2);
    }

    // but this next example is right back to a lifetime-related compiler error
    //let string5 = String::from("another very long string");
    //println!("string5 is: {}", string5);
    //let _result3: &str;
    //{
    //    let string6 = String::from("pdq");
    //    println!("string6 is: {}", string6);
        // this next line is part of the error
        // _result3 = naive_longest(string5.as_str(), string6.as_str());
        // error is: "borrowed value does not live long enough"
        // because string6 gets dropped at the end of this block
    //}
    // this next line is also part of the error
    // println!("The longest string is {}", _result3);
    // by the time we try to use result3 here, string6 has gone out of scope
    // so this _might_ be invalid (even though we know string5 is longer)
    
    // !!: string5 and string6 have *different* lifetimes, but our explicit
    // lifetime annotation used `'a` everywhere, meaning that everything must
    // have the same lifetime.  Hence the compiler won't let the example above
    // compile, because we're trying to call naive_longest with two strings
    // which have different lifetimes

    // the best option here is not to return a reference at all, but rather
    // to return *ownership* of a String instance. Now the inputs can have 
    // different lifetimes, and we don't need any lifetime annotations at all!
    fn longest(x: &str, y: &str) -> String {
        // the signature above uses `'a` _four_ (4!) times to clarify lifetimes
        // That's four references to 'a to tell the compiler that the three (3)
        // references must all share the _same_ lifetime.
        if x.len() > y.len() {
            String::from(x)  
        } else {
            String::from(y)
        }
    }

    let result4: String;
    let string7 = String::from("the rain in spain");
    println!("string7 is: '{}'", string7);
    {
        let string8 = String::from("lmno");
        println!("string8 is: '{}'", string8);
        result4 = longest(string7.as_str(), string8.as_str());
    }
    println!("The longest string is: '{}'", result4);
    
}

// struct fields can also have lifetimes
fn struct_lifetime() {

    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    // the compiler won't let this struct outlive the .part field
    // because that's a string slice, and you never really own a string slice,
    // since it's always just a reference to a (sometimes-hard-to-reach) 
    // underlying string

    let i: ImportantExcerpt;

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.')
            .next()
            .expect("Could not find a '.'");
        i = ImportantExcerpt { part: first_sentence };
        println!("Excerpt is: {:?}", i);
    }
    // but the lifetime of i is automatically the same as the lifetime of the
    // the `first_sentence` var (which ends up assigned to the `.part` field 
    // of the strutct). The lifetime of that var ends at the end of the block
    // above, which means the lifetime of i also ends at that time

    // thus this next line would be a compiler error if you uncommented it
    //println!("Excerpt is out of scope now: {:?}", i); 
}

fn main() {
    simple_scope();

    explicit_lifetime();

    struct_lifetime();

    // finally, note the special 'static lifetime, which is a one-off singleton
    // used to define static scalar constants which will live for the lifetime
    // of the program. These get inlined into the binary, and are not variables
    // in any sense of the word.
    let s: &'static str = "I have a static lifetime.";
    println!("Static lifetime string slice: '{}'", s);
    // note that the underling string slice is static... but the `s` variable 
    // is just a regular old variable without any special superpowers.
}
