/**
 * Rust branches work like they do in most languages.
 */
use rand::Rng;

fn main() {
    let number = rand::thread_rng().gen_range(0, 10);

    if number % 2 == 0 {
        println!("{} is even", number);
    } else if number == 5 {
        println!("{} is equal to 5", number);
    } else {
        println!("{} is odd but not 5", number);
    }

    // !! rust does not support truthiness!!
    // An {if} can *only* be followed by a bool!
    // The following will not compile: 
    //if number {
    //    println!("This would work if rust supported truthiness");
    //}

    // if is an _expression_, which means you can use it for assignment
    // Remember that any *block* evaluates to its final expression 
    // (And if the final line is not an expression, you cannot use it 
    // in an assignment statement at all, because Rust simply does not 
    // do anything like {nil} as a primitive value! 
    let result = if number % 2 == 0 {
        "even" // last line is expression sans semicolon, hence returned
    } else {
        "odd" // same
    };
    // both branches must have the same type! 
    // if you change either "even" or "odd" to a number, you can't compile

    println!("That random number was {}", result);

    // the `loop` keyword runs forever, 
    // so you'd better have a strategy to end it other than CTRL-C

    vanilla_loop();

    // the `while` keyword is mostly syntatic sugar for loop
    vanilla_while();

    // the rust `for` loop is a smart iterator, instead of being index-based
    vanilla_for();
}

// demonstrate the loop keyword
fn vanilla_loop() {
    let mut counter = 0; // `mut` means this is a _mutable_ variable
    // which means it can be changed, as opposed to simply being shadowed.
    // Shadowing is a mere parlor trick... true mutation requires {mut}. 

    // again we see that a *block* on the right is an *expression*, 
    // and so it can be the value for an assignment statement
    let result = loop {
        counter += 1;

        if counter == 10 {
            // a `loop` never ends until you call `break`
            break counter * 2;
        }
    };

    println!("vanilla loop result was {} (expected 20)", result);
    assert_eq!(result, 20);
}

// demonstrate the while keyword
fn vanilla_while() {
    let mut number = 3; // again, mutability is strictly *opt-in*

    // {while} is syntactic sugar over {loop}, with easier termination
    // You don't have to do your own if check within the loop, or call {break}
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    // on the other hand, you can't use a `while` block to assign,
    // which was a nifty little fringe benefit of using `loop`

    println!("LIFTOFF!!!");
    println!("After the loop, number was {}", number);

}

// demonstrate a vanilla for loop
fn vanilla_for() {

    // index-based iteration will always be error prone, so the rust `for` 
    // keyword doesn't support it. You can of course use `while` to do your 
    // own brittle index-based iteration, like so: 

    let a = [10, 20, 30];
    let mut index = 0;

    while index < 3 {
        println!("(while) index {} has value {}", index, a[index]);
        index = index + 1;
    }

    // but idiomatic rust is to use `for` with an *iterator*
    // most collections can give you an iterator out of the box
    for element in a.iter() {
        println!("(iterator) the value is: {}", element);
    }

    // and if you need the index, you get an *enumerator* instead 
    for (i, item) in a.iter().enumerate() {
        println!("(enumerator) index {} has value {}", i, item);
    }

    // finally, rust has easy access to numeric ranges  
    print!("Let's print a few numbers using the .. range operator: ");
    for number in 1..5 { // use '..' to create ranges... snazzy!
        print!("{}", number); //
    }
    println!("");
    println!(" NB: `1..5` does not include 5!");

}