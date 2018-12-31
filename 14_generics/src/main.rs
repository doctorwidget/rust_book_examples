/**
 * Generics are a big deal in static-typed languages. It's easy be unimpressed
 * by them at first glance if you've spent a lot of time working in dynamic
 * languages, since those languages are effectly 100% generic to begin with. 
 * 
 * But in static languages, you must declare the types of everything you use
 * up front, so if you write utility code that decorates or collects a target
 * type T, you have to specify that target type T in advance. That means there
 * is no obvious way to _reuse_ that utility code with target type U, short of
 * copying and pasting the source code and swapping out U for T. Copy plus 
 * paste plus edit is not actually "code reuse" in any useful sense.
 * 
 * __Generics__ are the solution for this problem. They allow strongly-static
 * languages to write reusable code. If the language has generics, the utility 
 * code from above could be written to work with _any_ target type. No need
 * to copy and paste anything -- you can reuse the utility code you wrote for
 * type T with types U and V and W and so on. 
 * 
 * Rust supports generics, hooray!
 */
// we'll want this for use with our generic `largest` function
use std::cmp::PartialOrd;

// Here's a pair of functions which are *not* generic. They solve the same
// exact problem, but since we are not using generics, we are forced to 
// essentially write the same code twice. Yuck!

// find the largest `i32` from an array slice of `i32` values. 
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// find the largest `char` from an array slice of `char` values
// nb: this is not DRY at all!
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// the *only* thing that differs for those two functions is their signature!
// The actual implementation is 100% identical, line for line and char for char
// This will never do!

// here is the GENERIC version of the two functions above
// here `T` stands in for our generic type, but note that it is not "any type".
// Because we use <T: PartialOrd>, it is "any type which implements PartialOrd"
// Thankfully, we only have to add the :PartialOrd once, inside the angle brackets.
// The angle brackets follow the method name and precede the arguments
// We could include multiples there (e.g <K, V> or <K: PartialOrd, V>, etcetera)
// Call this the "generics" clause, vs the more-familiar "arguments" clause
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}
// Finally, note that this version returns a _borrowed copy_ of T, `&T`,
// whereas the original Rust Book example returned an actual `T`, so the
// caller gained ownership of the returned value. It turns out that returning
// a `T` instead of a `&T` requires that you specify _two_ traits for T, 
// not just one. Rather than getting deeper into the weeds about traits in
// this chapter on generics, we'll leave this solution as is, and suggest
// you check out the improved version in `15_traits`. 

// Generic notation is not restricted to function signatures! You can use it
// inside a struct definition as well.
// This struct can take coordinates which are integer or floating-point 
#[derive(Debug)]
struct Point<T> { // here T always means a generic type, never a concrete one
    x: T,
    y: T,
}
// note that the generic clause has the same basic shape as it did when used
// with a function signature. And again, it appears right after the name.

// And Generic notation works with enums as well. This should be completely 
// unsurprising, since we've been seeing it since the earliest chapters.
// We were introduced to both Result<T, E> and Option<T> long before now.
enum _Result<T, E> { 
    // NB: "_" used in "_Result" to prevent compiler nagging about unused foo
    Ok(T),
    Err(E),
}
// NB: this also shows how you _mix_ types when you need to
// The Ok type T does not have to be the same as the Err type E. That's good,
// because a Result which could only refer to one generic type would be far 
// less useful than one in which you can specify any Err type you like. 

// A generics clause can declare *any* number of types. Having more than one
// or two will be terrible for the *clarity* of your code, but the compiler
// won't stop you. 
//     "If the implementation is hard to explain, it's a bad idea."

// With that in mind, let's end with an implementation which is hard to explain!
// Here's a second point type which accepts two different types for X and Y --
// so you could mix and match floats and integers (etc). 
// Start with the struct itself:
#[derive(Debug)]
struct Dot<T, U> {
    x: T,
    y: U,
}
// (NB: struct Dot<i32, i32> would *not* be legal: the compiler would try to 
// interpret i32 as a __generic__, and it would complain that it should have
// a name like "I32" instead of "i32". Keep that in mind for the next section:
// angle brackets in struct definitions *always* means generic types, but the 
// same angle brackets in an implementation block might mean a concrete type!

// Hence we have to use TWO generics clauses in the implementation block: the
// generic clause following the `impl` keyword tells the compiler to treat the 
// angle braces after `Dot` as generics, rather than concrete types. This is
// necessary because `impl Dot<i32>` would also be legal in this location, and 
// the compiler cannot tell which you mean out of the box. Adding the generics 
// clause after `impl` clarifies everything for the compiler. 
impl<T, U> Dot<T, U> {
    // it's generic blocks all the way down!
    // The next two types have a more limited scope than the <T,U> from above
    // They apply only within this nested method, and nowhere else
    fn mixup<V, W>(self, other: Dot<V, W>) -> Dot<T, W> {
        // NB!!: the incoming arg is `self` and not `&self`, which means this 
        // is a self-consuming method! The original instance is *destroyed* 
        // when this method is called on it! So if you call it without making 
        // it part of an assignment, the `self` is *moved*, then dropped at
        // the end of this function, and you're left with bupkis, DOH!
        Dot {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("(non-generic) The largest number is {}", result);
    let result2 = largest(&number_list);
    println!("(generic!) The largest number is {}", result2);

    let char_list = vec!['y', 'm', 'z', 'a', 'q'];
    let result3 = largest_char(&char_list);
    println!("(non-generic) The largest char is {}", result3);
    let result4 = largest(&char_list);
    println!("(generic) The largest char is {}", result4);

    let integer_point = Point { x: 5, y: 10 };
    println!("Point struct with integers: {:?}", integer_point);
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("Point struct with floats: {:?}", float_point);

    // the Dot type allows different types for T and U
    let d1 = Dot { x: 5, y: 10.4 };
    // they don't even have to be numeric!
    let d2 = Dot { x: "Hello", y: 'c'};

    // as noted, the mixup() method *consumes* the callee!
    let d3 = d1.mixup(d2); // d1 is now invalid
    println!("Dot struct after mixup(): {:?}", d3);
    //println!("d1 is now invalid, DOH: {:?}", d1); // compiler error  
    // "value borrowed here after move"... .mixup() is a _move_!!  

}
