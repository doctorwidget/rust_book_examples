/**
 * Enums allow you to mix and match different data types, but as a set of
 * __choices__, where each instance of the Enum is always one and only one
 * of the available choices. Compare that to structs, which also allow you
 * to mix different data types, but which require each instance to have one 
 * value for each end every key within the struct definition. Both of these
 * data structures allow the aggregation of mixed data types, but they do so
 * in a way which is completely orthogonal to (or inverted from) the other.
 * 
 * Other languages (e.g. Python or Golang) have Enums whose choices are 
 * limited to simple scalar values (sometimes even just numbers!). That 
 * limits their  utility compared to Rust enums, which can use the entirety 
 * of the Rust type system as building blocks, mixing and matching types as 
 * needed. Rust enum choices can have no associated value, or be associated
 * with a wrapped constant, or a wrapped variable, or a wrapped tuple, or 
 * even a wrapped struct instance! This gives them tremendous flexibility 
 * and power compared to enums based on a simple scalar constant range.
 * 
 * Because of this power, Rust Enums are _ubiquitous_. They are used to 
 * robustly handle concerns which are very tricky in other languages, such 
 * as safely managing the possibility of null values and errors, or routing 
 * a variety of incoming argument types from a single function entry point.
 * Enums are used _all over_ Rust, so you must get familiar with them! 
 */


// Here's an Enum with four _choices_ (aka `variants`)
// An instance of this enum will always be one and only one of these variants.
// The associated values are _variables_ and not _constants_!
// Enums are not *required* to have associated values: note that Quit has none.
enum Message {
    Quit,   // variant with no associated value
    Move { x: i32, y: i32 }, // a struct-style variant
    Write(String),   // variant with a single big-S string value
    Color(i32, i32, i32), // a tuple-style variant
}

// Enums can use __impl__ blocks just like structs do. That means they are 
// first-class participants in Rust polymorphism, hooray!  
// However, note that while Message is a first-class *type*, each individual
// variant is *not* a first-class type. That means you cannot have separate
// implementations for each variant: the whole Enum gets the method, and you
// must use a match statement inside the method to route the result. 
// Fortunately, Match statements are awesome, so this is not a big limitation!
impl Message {
    fn call(&self) {
        // The correct way to discriminate Enum variants is a `match` statement.
        // This is the _only_ idiomatic way to do it!
        match self {
            // NB: it seems you have make *namespaced* references to the variants
            // I wonder if I'm missing a terser way to do this?
            Message::Quit => println!("I am a Quit"),
            // NB#2: you must destructure x and y as 'x' and 'y'... there is no
            // concise way to tive them different local names. After all, this 
            // is the whole point of using a struct in the first place!
            Message::Move{x, y} => println!("I am a Move: {}, {}", x, y),
            // but a single-value variant can get any local name you like
            Message::Write(foo) => println!("I am a Write: {}", foo),
            // as can the sequential fields from a tuple variant
            Message::Color(r, g, b) => println!("I am a Color: {}, {}, {}", r, g, b)
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    California,
    Colorado
    // --snip--
}

// enum of some coin types
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState), // Quarter is the only variant with an associated value
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            // a match clause gets a full-blown block all its own,
            // so you aren't limited to just returning one value
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}


fn main() {
    let m_quit = Message::Quit;
    let m_move = Message::Move{x:5, y:7}; // structs require explicit key & value! 
    let m_write = Message::Write(String::from("hello"));
    let m_change = Message::Color(255, 200, 100);
    
    m_quit.call();
    m_move.call();
    m_write.call();
    m_change.call();

    // in fact, we already met Enums long ago, wa back in the guessing_game 
    // chapter, where we wrote `match` clauses based on the standard-library 
    // Option<T> type. This Enum is so fundamental that it's practically a 
    // language keyword. Any type at all can be wrapped in a // Option<T>.
    // The only two variants are Option::Some(foo), where foo is a T instance,
    // or Option::None, where None has no associated value. In Rust you never
    // ever need to check for null||nil||undefined||None, but it is absolutely
    // routine to write match clauses for Option::Some || Option::None.
    
    // Because Option<T> is so important, it is part of the "prelude", which
    // means the rust compiler automatically makes it available even if you 
    // don't ask for it by name. Hence you'll see code all over that has 
    // match clauses for Some || None, without the namespacing of `Option::`.
    // This is even more encouragement to think of them as language keywords!
    // But they are not: they are plain old generic Enum variants. 
    let s_five = Some(5); // the compiler auto-types this as an Option<i32>
    let s_none : Option<i32> = None;
    // and you cannot send an Option<T> straight to println!
    // println!("S_five is: {}", s_five);
    
    fn maybe(candidate: Option<i32>) {
        match candidate {
            None => println!("Nuthin' to see here"),
            Some(x) => println!("That's a {}", x)
        }
    }
    maybe(s_five);
    maybe(s_none);

    // the compiler complains about every uninstantiated type, yeesh!
    // and then it complains if they are instantiated and unused, double yeesh!
    // But it won't complain if you instantiate them with underscores, ok then!
    // Finally, note that we're going to feed these to functions that feed 
    // their arguments to println!, which means we face the full fury of the
    // borrow checker. Hence the borrow types based on `&`
    let _alabama = &UsState::Alabama;
    let _alaska = &UsState::Alaska;
    let _california = &UsState::California;
    let _colorado = &UsState::Colorado;

    let c_penny = Coin::Penny;
    let c_nickel = Coin::Nickel;
    let c_dime = Coin::Dime;
    let c_quarter = Coin::Quarter(UsState::California);
    println!("A penny is worth: {}", value_in_cents(c_penny));
    println!("A nickel is worth {} cents", value_in_cents(c_nickel));
    println!("A dime is worth {} cents", value_in_cents(c_dime));
    println!("A quarter is worth {} cents", value_in_cents(c_quarter));

    // out of the box, all match statements must be __exhaustive__,
    // meaning you need one arm clause for every single variant. 
    // But sometimes you are genuinely only interested in a subset of
    // all possible variants. There are two workarounds to help keep
    // things concise in those situations. The first one is to use a
    //final `_` as the last arm caluse; this will match everyone else.
    fn is_california(candidate: &UsState) {
        match candidate {
            UsState::California => println!("California here I come!"),
            // all other states match here
            _ => println!("Just another state which is NOT California...")
        }
    }
    is_california(_colorado);
    is_california(_california);

    // `if let` is the the second option, which comes at the problem from the
    // opposite direction. Rather than saying "everything not named yet", it 
    // homes in on a single valid candidate, and says "this and only this"
    fn only_california(candidate: &UsState) {
        // Here we have to specify both sides of the = test explicitly
        // This is unlike `match`, where there's an implied = test 
        // (Technically you could do any comparison that yields a true bool,
        // remembering that there is no such thing as truthiness in Rust)
        if let UsState::California = candidate {
            println!("California here I come!!");
        } else {
            println!("Oh, it's some other state. Meh.");
        }
    }
    only_california(_alabama);
    only_california(_alaska);
    only_california(_california);

    // so by default, match is 100% exhaustive, which can be exhausting.
    // Together, `_` and `if let` give you two ways to write more-concise
    // match statements. One is in the spirit of "everything not forbidden
    // is allowed", and the other in the spirit of "everything not allowed
    // is forbidden".
}
