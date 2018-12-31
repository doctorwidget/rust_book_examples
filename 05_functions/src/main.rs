/**
 * Nothing too surprising here. Function signatures must include types,
 * just as in most other typed languages. 
 * 
 * But in contrast to many typed languages, understanding Rust functions 
 * requires understanding the difference between statements and expressions.
 *
 * An _expression_ is _evaluated_ (like pretty much everything in Clojure!)
 * Expressions do not need to end in a semicolon. Expressions are evaluated
 * to a final _value_. Just break down the word "evaluated"!
 * 
 *     expression = evaluated
 * 
 * If the function ends in an _expression_, the value of the expression is 
 * the return value for the function.  
 * 
 * In contrast, a _statement_ is called for its *side effects*, and it does
 * not evaluate to anything at all. As a final quirky little detail, all
 * statements *must* end in semicolons. 
 * 
 *     statement = (side effects) && semicolon;
 * 
 * Since statements evaluate to nothing, if the final line of a function is 
 * a statement, the function returns nothing. 
 */
fn main() {
    another_function(5, 6);

    statement_demo();

    let z = func_with_return(); 
    println!("The value of z is: {}", z);

    let w = another_return(z);
    println!("The value of w is: {}", w);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// note that rustfmt strongly prefers snake_case to camelCase,
// at least for function names
fn statement_demo () {

    // the next line is an error and will not even compile: 
    // let x = (let y = 6);
    // the right side doesn't return anything, so it cannot be assigned to x
    // thus you can never do x = y = 6, which is legal in C, Ruby and JS
    // That's fine! I have always hated lines like that anyway.

    // So let's try that again:
    // here the right side is a _block_
    // and it turns out that blocks evaluate to their final line,
    // just like a whole function does!
    let y = {
        let x = 3;
        // this expression is the value for the block
        x + 1
    };
    // and now y should be 4

    println!("The value of y should be 4, and it is: {}", y);
}

// the other two functions here end in statements and so return nothing
// This one ends in an expression, which is _implicitly_ returned. 
// You could add a 'return', but it is completely unecessary.
// The '->' in the function signature denotes the return type 
fn func_with_return () -> i32 {
    // 42 is a valid expression, so it doesn't need a semicolon either!
    42
}

// Another example of a function with a return 
fn another_return (i: i32) -> i32 {

    // if you put in the semicolon, you are *enforcing* a statement 
    // and a statement evaluates to nothing, 
    // so this is a compiler error! 
    // i * 2; 

    // but throw in an explicit return and we would be good again
    // return i * 2;

    // so the semicolon is not some kind of loosy-goosy optional... 
    // if it is there, it *enforces* statement-ness 
    // and expressions *cannot* have it 
    i * 2
}