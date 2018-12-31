fn main() {
    // fails cargo check because we never said x was mutable!
    // let x = 5;
    // but this works because now the compiler knows x can change
    let mut x = 5;
    println!("The value of x is: {}", x);

    // now we can _reassign_ the value of x
    x = 6;
    println!("The value of x is: {}", x);

    // alternatively, we can _redeclare_ a value, aka "shadow" it
    // this works even without the `mut`, because we are declaring
    // a brand new variable object, which happens to have the same
    // name as the first one
    let y = 2;
    let y = y + 1;
    let y = y * 3;
    println!("The value of y is {}", y);

    // using a steady stream of _redeclarations_ lets you get around
    // the default immutability of variables, so you can have just one
    // sensible variable name, and still end up with immutability once
    // you are done with the intermediate calculations

    // better yet, with a steady stream of redeclarations, we can even
    // change the _type_ of the variable!
    let spaces = "  "; // from a string
    let spaces = spaces.len(); // to a number

    // whereas if the variable was mutable, we couldn't change the _type_
    let mut altSpaces = "  "; // a _mutable_ string, but always a string
                              // so this next line cannot compile
                              // altSpaces = altSpaces.len(); // because it would change the _type_

    //// epilogue (or prologue to the section on functions)
    // note that everything above is a _statement_ and not an _expression_
    // this matters a lot in the next section of the rust book, on functions,
    // where an _expression_ in the last line is a valid return value,
    // but a _statement_ on the last line is _not_ a valid return value

    // statements are called for their *side effects*, and end in semicolons

    // expressions are *evaluated*, and end in with emptiness (no punctuation)!

    // so the program above is a series of statements,
    // but as soon as you throw in an _if_ expression, it is _evaluated_,
    // and does not need to be terminated by a semicolon.

    // even more of an epiphany: most of the statements above are actually
    // compound entities, with an _expression_ on the right, whose evaluated
    // value is assigned to the variable on the left. The whole thing is a
    // statement, but they all contain expressions. The whole point of the
    // oh-so-fundamental `=` operator is just to transfer evaluated expressions
    // into variable boxes on the left.
}
