/**
 * Rust errors come in two flavors:
 * 
 * - recoverable: i.e. a routine IO error (... handle with Result<T, E>)
 * - unrecoverable: i.e. wtf? (... handle with the `panic!` macro)
 * 
 * The former is handled via Result<T, E> enum values, as we saw early on. 
 * Result<T, E> variants are absolutely ubiquitous as return values, and the
 * caller is expected to unpack them with the ever-useful `match` keyword.
 * 
 * The latter might be triggered by the system, or you can trigger it 
 * intentionally with the `panic!` macro. Even in panic! mode, Rust maintains 
 * an orderly composure, offering you ways to customize the error output.
 * 
 */ 

// io operations are always a reliable way to demonstrate errors
use std::fs::File;
use std::io;
use std::io::prelude::*; // required for the read_to_string method
use std::io::ErrorKind;

// this function will definitely trigger a panic
pub fn eek() -> i32 {
    let v = vec![1, 2, 3];
    v[101]
}

// and so will this one
pub fn demo_io_simple() {
    // important note: "hello.txt" is assumed to be a sibling of Cargo.toml
    let _f = File::open("hello.txt"); // f now has an Err(E)
    // so we're definitely going down the Err path, not the Ok path
    let mut f = match _f { 
        Ok(file) => file,
        Err(error) => {
            // the demo is OVER at the point where it panics - which is why
            // we've wrapped this in a function. Easier to comment out one line
            // calling this function than to comment all of this out
            panic!("Problem opening file: {:?}", error)
            // runtime panic: 'Problem opening file: Os { code: 2, kind: NotFound ...}
            // note that the error object is a struct with details, not a plain str

            // What if we wanted to avoid ending the demo here? We cannot just
            // do a println! about it instead of `panic!`, because the compiler
            // immediately notices that this arm does not return a return value
            // of type std::fs::File. There's no easy way around that mismatch!
            // println!("Caught an expected error: {:?}", error);
        },
    };
    // we won't get this far, but let's play the scenario out anyway
    let mut contents = String::new();
    // because it's a nice demo of getting a file contents into memory 
    // interestingly, you do this imperative mutation of an external variable
    // (which seems like a real throwback to all-mutable all-the-time coding)
    f.read_to_string(&mut contents).unwrap(); 
    // nb: the above could error out too, which is why we call .unwrap() on it
    // unwrap() tells the compiler: yes I know that's a Result, so do one of
    // two things:
    // 1) call an (unhandled!) `panic!` if there is an error
    // 2) unwrap the Ok(T) if there is one, and assign it (if this is an assignment)

    // and we definitely never get here
    println!("File contains: {}", contents);
}

// this one has smarter error handling, showing a boolean tree inside the match
pub fn demo_result_smarter() {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(ff) => ff, // outer match resolves to the *found* file `ff`
        // error.kind() gives us the particular error flavor
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // !!: "hello.txt" will be created as a sibling of Cargo.toml
                Ok(fc) => fc, // outer match resolves to the *created* file `fc`
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
    // in this case, we make it through the nested tree above, creating the 
    // file the first time this runs, but then it errors out at the unwrap() 
    // Then oddly, on the second trial, it works, and prints the empty file contents
    // So you have to delete the "hello.txt" file to see the error
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    println!("File contains: {}", contents);
}

// this one uses expect() instead of unwrap like the prior two
pub fn demo_result_expect() {
    // expect should be read as "expect an error"
    // it lets you provide a more detailed error in the stack trace
    // Otherwise, expect() is similar to unwrap(), assigning to the left side
    let mut f = File::open("spam.txt").expect("spam spam spam spam");
    f.write_all(b"eggs").expect("Won't get here - won't see this message");
}

// Try to return a username as a String from a file, OR return an Error
// Since the return type is Result<T, E>, callers must be prepared
// to handle both possibilities!
pub fn read_username_verbose() -> Result<String, io::Error> {
    let f = File::open("users.txt");

    // first match is an *assignment* - we're trying to give `f` a value
    let mut f = match f {
        Ok(file) => file,   // f is now assigned a value of `file`
        Err(e) => return Err(e), // !! early return !! Error propagates!
        // the return value for the entire function is the Err(e) value
        // because `return` is never about assignment within the function:
        // it always and only means the whole function returns early
    };

    let mut s = String::new();

    // the second match is an *expression* at the final line of the function,
    // so whatever it evaluates to *is* the return value for the function
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s), // evaluate to Ok(s), which is a valid variant 
        // so the entire function returns Ok(s) in this case
        Err(e) => Err(e), // evaluate to Err(e), another valid variant,
        // and the entire function returns Err(e) in this case
    }
}

// this fn has the exact same logic as the one above, 
// but it is much more terse, and also more idiomatic
// It's also our introduction to the `?` keyword
pub fn read_username_terse() -> Result<String, io::Error> {
    // Think of the `?` as being very macro-like. Wherever it appears it ends
    // up doing what the verbose version did above, which is to say:
    // (1) this entire function does an early `return` if the Result is Err(e)
    // (2) the Err(e) we just discovered is used as the function return value
    // (3) otherwise unwrap the Ok(T) value and assign it to the left side
    // So _this_ seems like it's what should have been called "unwrap()":
    // it unwraps and assigns the value in the Ok(T) scenario, 
    // and quietly propagates the Err(E) in the other scenario,
    // saving 3 extra lines every time a (ubiquitous!) Result pops up
    let mut f = File::open("users.txt")?; // first use of `?` saves 3 lines
    let mut s = String::new();
    f.read_to_string(&mut s)?; // second use saves 3 more lines
    Ok(s) // explicitly return `s` now that we are confident that all is well

    // tl;dr: you're going to see `?` all over the place, so get used to it
    // It always results in either *assignment* or *propagation*
    // - if Ok(T), the *assignment* proceeds as expected to the left-side var
    // - if Err(E), that exact Err(E) case is *propagated* via an early return

    // NB: this can *only* work if the enclosing fn already has a return type
    // that matches Result<T, E>, because the early return will happen for the
    // entire enclosing function! That's the inevitable natural consequence of 
    // *propagating* errors: you are just passing the buck, and someone else 
    // somewhere else will ultimately have to deal with it. So this doesn't do
    // anything to *solve* Result<T, E> overload... it merely *shifts* it.
}

// a smart Guess struct that panics if someone (i.e. a user playing a game)
// submits a guess outside the range of 1 to 100. That UX leaves something to
// be desired, but it's a good *structural* demonstration of both intentional 
// use of `panic!`, and a classic read-only getter method. 
pub struct Guess {
    value: i32, // value is _private by default_, remember!
}

impl Guess {
    // which means users must call Guess:new(n), rather than Guess { n }
    pub fn new(value: i32) -> Guess {
        // nb: the absence of a &self argument makes this a static method
        // versus an instance method. Thus it must be invoked as Guess::new(i) 
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    // a classic getter - so users can _read_ the Guess value, without ever
    // being allowed to _write_ to it
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!(" ... Error demo module: all demos commented out by default ...");
    println!("Demo a custom panic! (uncomment next line to see)");
    // panic!("Something has gone terribly terribly wrong");
    // the default output is minimal, unless you set an env variable
    //     export RUST_BACKTRACE=1
    // and after that you'll get a nicely numbered stack trace

    println!("Demo a native panic! (uncomment next line to see)");
    // uncomment the call to see the eek function from above panic
    // let x = eek();
    // runtime panic: 'the len is 3 but the index is 101'

    // Remember that Result<T, E> is always available in the prelude, with 
    // exactly two (2) variants: Ok(T) or Err(E). 
    
    println!("Demo a simple IO error (uncomment next line to see");
    // uncomment the call to see the error happen
    //demo_io_simple();
    
    println!("Demo a smarter IO error (uncomment next line to see");
    // uncomment to see yet another error type, this time with the .unwrap()
    // demo_result_smarter();

    println!("Demo the .expect() helper (uncomment next line to see");
    // uncomment to see the use of .expect() to give a good error message
    // demo_result_expect();

    println!("Demo manual error propagation (uncomment next line to see");
    // uncomment out to see a verbose example of error propagation
    // Since the _function_ propagates the error, the _caller_ is the one that
    // has to handle it... and that's us right here in main()
    // let _oops = read_username_verbose().expect("I expect this failed");

    println!("Demo terse error propagation with `?` (uncomment next line to see");
    // similar logical flow, but much more idiomatic
    // let _oops2 = read_username_terse().expect("I expect this failed... tersely");

    // This Guess is valid and works great!
    let g1 = Guess::new(50);
    println!("Guess value is: {}", g1.value());
    // but both of these next two would cause early panic if uncommented 
    //let g2 = Guess::new(-1); // panic!
    //println!("Guess2 value is: {}", g2.value());
    //let g3 = Guess::new(2000); // panic!
    //println!("Guess3 value is: {}", g3.value());
    
    println!("... Error demo module complete ...");
}
