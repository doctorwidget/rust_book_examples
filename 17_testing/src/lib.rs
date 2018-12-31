/**
 * Library file for testing purposes
 */

pub fn echo(msg: &str) {
    println!("{}", msg);
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// this can be tested by external code because it is `pub`
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

// but this can be tested only in this file, or a submodule of this file
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
pub struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess {
            value
        }
    }
}

// testing begins here!

// All output to stdout is swallowed by default *unless* the test fails
// For a failing test, you'll see all the stdout for that test.
// But stdout for successful tests is "captured" (aka swallowed).
// To see the stdout for successful tests (why?), use this flag:
//
//      cargo test -- --nocapture
//
// you can run individual tests by name like so:
//
//      cargo test experiment   
//
// This does a regexp match, so it should catch the 'experimentation' test
// Meanwhile, this should catch both 'exploration' and 'experimentation'
//
//      cargo test ex
//
// and so on and so forth
//
// You can explicitly run *all* tests flagged with `#[ignore]` like so
//
//     cargo test -- --ignored      # runs both `bad_math` and `fail_me`
// 
// And finally, you can combine all of the above to test *some* ignored tests
//
//     cargo test bad -- --ignored   # runs only `bad_math` 
//

#[cfg(test)] // annotation that this entire module is test-only
mod tests {
    // reminder#1: a module cannot see its parent scope by default
    use super::*; // using `*` is usually sketchy, but this seems legit
    
    // reminder#2: once brought into scope, the parent scope is automatically public
    // So we can test the internal_adder function, even though it is private
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }

    #[test] // annotation that this specific test is test-only
    fn exploration() {
        // assert_eq! is one of the main workhorses for testing
        // it takes two values and asserts their equality
        // nb: on failure, the Rust test output will use `left` and `right`
        // instead of the more-common `expected` and `actual`
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn experimentation() {
        // assert_ne! is another workhorse
        assert_ne!(3, 4);
    }

    #[test]
    fn evaluation() {
        // assert! is yet another workhorse:
        // it takes one expression which must evaluate to a boolean
        assert!(1 > 0);
        // remembering, of course, that Rust has no concept of "truthiness",
        // so only `true` or `false` need apply
    }

    // note that this our first example of _two_ annotations
    // You can just place them sequentially in a line, easy peasy
    // (it is also legal to use a vertical stack)
    // This test always fails, so we surely don't want it routinely running
    #[test] #[ignore]
    fn fail_me() {
        assert!(0 > 1);
    }

    // a second ignored test, to test running subsets with the `--ignored` flag
    #[test]
    #[ignore]
    fn bad_math() {
        assert_eq!(0, 1);
    }

    // On the other hand, you might want to explicitly test for panic!
    // You do that with the #[should_panic] annotation
    // Note that here we show the vertical stack mode for annotations
    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("I meant to do this");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        // note the use of an inner `!` to negate the outer `assert!`
        assert!(!smaller.can_hold(&larger));
    }    

    // And of course you can add failure messages as an extra parameter
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol"); // change this to see a failure
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`", result
        );
    }   

    // we saw `#[should_panic]` up above, but we only used its simplest form
    // You can also specify an expected panic message
    // Here we say which message we expect for this particular *type* of panic
    // alter the guess to valid, and the test fails (with no extra info)
    // alter it to invalid but under 1, and the test fails with extra info
    // this is how you test for the wrong *type* of panic!
    #[test]
    #[should_panic(expected = "value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }   

     // And here we specify the opposite
     // alter the guess to valid, and the test fails
     // alter the guess to invalid and over 100, and the test fails with extra info
     // again, this lets you test for the wrong *type* of panic!
    #[test]
    #[should_panic(expected = "value must be greater than or equal to 1")]
    fn less_than_one() {
        Guess::new(-1);
    }     
    // NB: the expected strings get automagic substring matching, so you 
    // do not need to include the complete expected panic string

    // tests that use Result<T, E>
    // This is _obviously_ going to be a super common requirement!
    // To accomplish this, you add a *return value* to your test function signature.
    // It should always have this type: Result<(), String>
    #[test]
    fn result_t_e_wins() -> Result<(), String> {
        // note the return value of Result<(), String> in the function signature
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
