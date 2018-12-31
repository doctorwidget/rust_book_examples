// a guessing game
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // printing the secret number is useful during development,
    // but does not make for the best gameplay
    // println!("The secret number is {}", secret_number);

    // a loop statement all by itself is infinite!
    // this only ends when we reach our `break` statement below
    // (or via CTRL-C, or by entering a non-number)
    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        // compiler infers that `guess` is a string because we gave it a string!
        // all user input from stdin() is also a string

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // nb: read_line returns a Return object, which is a core Rust type
        // it is an enum which is either OK or Err (short for Err)
        // You can handle this much like we do with a JS promise... via
        // attaching handlers. The .expect() handler will fire for an Err,
        // If the enum is not Error, it must have been an OK value, and that
        // OK value gets placed in the `guess` variable

        // (Now, back to the main flow of control after our Return detour)
        // But wait, we generated an integer for `secret_number`, doh!
        // so we must convert the guess from a string into a number
        // The next line is looks like we are _redeclaring_ the variable,
        // but Rust considers this an instance of "shadowing". Ok then!
        let guess: u32 = match guess.trim().parse() {
            // it turns out that using expect() on a Return object is kind of 
            // the lazy way out... or at best the quick and dirty way out
            // The more-correct approach is to feed the Return object to a
            // {match} statement, and then explicitly handle both the
            // OK case and the Error case with actual branches, like so:   
            Ok(num) => num, // okay means we should have a number, so resolve to it
            Err(_) => continue, // lazy but *explicit* handling of an error
            // typical use of `_` for "a variable I don't care about",
            // and then the language keyword {continue} to keep the loop going
            // instead of crashing the program
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } // end Equal match clause
        }// end entire match statement
    } // end loop
            
    println!("Congratulations!");
}// end program
