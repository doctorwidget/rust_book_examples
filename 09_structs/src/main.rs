/**
 * Structs are used a *lot* in Rust, because they are an extremely lightweight
 * way to create organized data structures in the absence of classes per se.
 * Structs are ubiquitous in Rust, the same way that anonymous object hashes
 * are ubiquitous in JS. 
 * 
 * The benefit of the Rust approach is that structs are a full-fledged type, 
 * despite being only a tiny bit more verbose to create than an anonymous JS 
 * hash is. Since a named struct is a truly first-class type, it can be used 
 * as part of Rust `match` statements and inside Rust Enum definitions, which
 * are two killer features found in Rust and not JS. It also allows them to 
 * participate in the Trait pattern, which is how Rust implements full-blown
 * polymorphism despite lacking classes.
 * 
 * In short: structs offer all of the advantages of classes without any of
 * their disadvantages. They are awesome. Learn them, live them, love them.
 */

/** 
 * Define a struct as a top-level, re-usable, shareable data structure.
 * But this is not a *class*: it has no _methods_, and it cannot _inherit_.
 * It is simply a named dictionary type whose values can be of mixed types. 
 * Rust also has _maps_, but they can't have mixed value types. And Rust also 
 * has _tuples_, which can mix value types, but which don't have named keys. 
 * The struct thus fills an essential logical hole. 
 */
#[derive(Debug)] // This is an annotation. It is completely optional (see below)
struct User {    // this is the actual struct definition
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
// Without the annotation above, this struct would not be debuggable (i.e. it 
// could not participate in various utility macros like println! and format!).
// Rust annotations are much like those found in other languages (e.g. Python).
// They mean the struct will end up with a little extra somethin' something'
// above and beyond the literal definition that follows. What exactly is that
// somethin' somethin'? It depends on the annotation: see the docs for each!

// there are several ways to provide defaults, but none of them involve
// specifying an inline default inside the struct defintion. The simplest
// one is probably to just have a factory for creating struct instances
// which takes the parameters that have no default, like so:
fn build_user(email: String, username: String) -> User {
    // A struct instantiation is an _expression_, so it qualifies as a
    // valid return object if it is the last thing in the function
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    } // look ma! no semicolon!
}

// A "tuple struct" is a struct with numbered fields, like a tuple but named.
// In fact, it seems to me that a better name for this would be "named tuple"
// or "typed tuple". These behaves like a tuple for property access, but they
// are first-class types, giving them all the benefits described earlier: they
// are usable in match statements, in Enum definitions, and with Traits. 
#[derive(Debug)]
struct Color(u8, u8, u8); // access fields as foo.0, foo.1, and foo.2

// Best of all, we can associate _methods_ with structs after defining them.
// This is the aforementioned replacement for polymorphism, which obviates
// the need for a heavyweight class system. 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Methods are layered onto the struct *after the fact*, in an `impl` block.
// The key word of interest here is __impl__; it can only ever be used *after* 
// the following type has been defined or otherwise brought into scope
impl Rectangle {
    // NB: the Python-esque '@self' reference is *not* optional!
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // we can now call foo.area() on Rectangle instances

    // a utility method so one Rectangle can compare itself to another
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // a __static__ method for the Rectangle *namespace*
    // This is not an _instance_ method: note the lack of `&self`!
    // This must be invoked as `Rectangle::square(12)`
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
// This is the same basic pattern we have seen in Clojure with Protocols.
// it allows us to associate arbitrary methods with arbitrary types without 
// having to alter the internals of the type. We don't even have to have any
// *access* to the source code of the type! That makes this approach far more 
// flexible and broadly useful than any heavyweight class hierarchy. Structs
// are thus "open for extension but closed for modification". Hooray!

// for example, we can add a _method_ to the Color struct down here even
// though we defined it way up there. And we're not limited to doing this within
// the same file where the struct was defined: we could use this approach to add
// methods to structs from other libraries/modules that we did not even write.
// This means that multiple `impl` blocks are 100% A-OK. You can have as many 
// such blocks as you need in whatever locations that you need. 
impl Color {
    fn rgb_str(&self) -> String {
        // this macro creates a big-S String
        format!("(red: {}, green: {}, blue: {})", self.0, self.1, self.2)
        // nb: remember that adding a semicolon to the line above makes it
        // a statement instead of an expression, which prevents it from being
        // the return value!
    }
}

fn main() {
    // Intantiating a struct is *exactly* like creating an anonymous JS hash, 
    // except that you precede the opening brace with the struct type name.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // or we can use the factory function we defined up above
    let user2 = build_user(String::from("foo@bar.com"), String::from("Bob"));

    // Another way would be to define defaults for _everything_, and then use 
    // the ES6-style object literal creation syntax to fill in all missing fields. 
    let du = build_user(String::from("anon@example.com"), String::from("anonymous"));

    // now we can use "du" as the default and unwrap bits of it into new instances
    let user3 = User {
        active: false,
        .. du   // unwrap the fields of the default user here
    }; // again, this is the same pattern seen in ES6 destructuring

    println!("Hello, {}, {} and {}", user1.username, user2.username, user3.username);
    
    // here we show the payoff for including '#[derive(Debug)]' up above
    println!("{:?}", user3);

    // interestingly, tuple structs are still defined with smooth braces,
    // just like regular tuples. You might think they would use curly braces,
    // like other structs, but they do not. I see this as further evidence
    // that "named tuple" or "typed tuple" would both be better names.
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    println!("Black is: {:?}", black);
    println!("White is: (red: {}, green: {}, blue: {})", white.0, white.1, white.2);

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("The area of rect1 is: {}", rect1.area());
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // demo our after-the-fact implementation of .rgb_str on Colors
    println!("Use of rgb_str() method on black instance: {}", &black.rgb_str());

    // demo of the *static* square method
    let square = Rectangle::square(12);
    println!("A simple square: {:?}", square);

}
