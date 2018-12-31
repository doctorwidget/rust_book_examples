/**
 * This is the one and only library module.
 * See `{root}/Cargo.toml` for more details
 */

// we'll be using these traits from the standard library in fugly_fn
use std::fmt::Display;
use std::fmt::Debug;

// sanity test to be used by other files
pub fn greet() {
  println!("Hello From My Library!");
}

// A trait definition
pub trait Summary {
  // Functions inside a trait definition are signature-only: no body needed!
  // This is 100% standard for interfaces, which traits are +/- synonymous with
  fn summarize(&self) -> String;

  // but you have the __option__ to provide a default implementation 
  // If you do this, the implementors have the option to override or not
  fn more(&self) -> String {
        String::from("(Read more...)")
  }
}

// a struct, which is always defined in terms of its data fields, 
// and never in terms of its methods or traits
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// now we implement the Summary trait for the NewsArticle struct
impl Summary for NewsArticle {
    // take &self instead of self to avoid consuming the struct
    fn summarize(&self) -> String {
        // format! is much like println!, but it returns a big-S String
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // no implementation for .more()... so we use the default
}

// another struct, with (almost) entirely different fields.
// Only `.content` overlaps between Tweet and NewsArticle
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
// but there's nothing stopping Tweet from implementing Summary, 
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    // override the default implementation for more
    fn more(&self) -> String {
      String::from("(Tweet more...chirp chirp!)")
    }
}

// traits are a first-class type, so they can appear in function signatures
// Here's one form for using a trait inside a function signature: 
pub fn notify_a(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// Here's another form, with the exact same effect
pub fn notify_b<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
// the longer form takes only one extra char but seems much clearer to me
// fun fact:  traits, structs & enums are the only 3 user-definable types
// fun fact #2: the second form above is sometimes called a "trait bound"

// fun fact #3: you can use '+' to concatenate traits in both forms above
// fn notify_a(item: &impl Summary + OtherTrait); 
// fn notify_b<T: Summary + OtherTrait>(item: &T);

// fun fact #4: you can use a `where` clause if signature is getting so 
// complicated that it's no longer readable on a single line. (But shouldn't
// you think about refactoring things so you don't have that problem?)
pub fn fugly_fn<T, U>(t: T, u: U) -> i32
    where T: Display + Debug,
          U: Clone + Debug 
{
  // nb: Debug (not Display) is the trait required for use with println!
  println!("Wow, this is fugly! {:?}, {:?}", t, u);
  42
}

// Being first-class means you can also *return* a trait
// But the syntax is slightly different from a normal return
// The signature in the next function specifies `-> impl Summary`, whereas you
// might think you could simply specify `-> Summary`
pub fn returns_summarizable() -> impl Summary { 
    Tweet {
        username: String::from("spammy_mc_spammer"),
        content: String::from("tweet tweet tweet tweet tweeeeeeeet"),
        reply: false,
        retweet: false,
    }
}
// but oddly, you can't do any kind of branching here (match or if etc)
// to return more than one possible concrete implementation. 
// There's a way to get around this, but it's relegated to an advanced topics
// section of the rust book, and we won't cover it here.
//        https://doc.rust-lang.org/book/ch17-02-trait-objects.html

// we can put all this together and do a final version of the largest function
// from `14_generics`. The Rust Book doesn't actually solve that problem: I did
// my own research to get the solution shown there, which uses the `PartialOrd`
// trait. But my solution returns a _borrowed reference_ to the type, `&T`, 
// rather than returning an owned copy. Since ints and chars are both copyable,
// we ought to be able to make largest() return a true `T` reference, so the 
// caller owns it free and clear. And we can do exactly that, by specifying 
// the Copy trait along with the `PartialOrd` trait. 
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
// and now you can return a plain old `T` instead of the `&T` I had to use
// in my version from `14_generics`. 

// Finally, you can do _conditional_ implementations for a type.
// For example, consider this Pair struct, which works for any two instances
// of the type T
pub struct Pair<T> {
    x: T,
    y: T,
}

// All the methods in this block are implemented universally... we don't
// do any special checking on the type T
impl<T> Pair<T> {
    // so ALL Pair<T> instances can be created with Pair<T>::new()
    pub fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }

}

// But this next implementation block is *conditional*
// Not all Pair<T> instances have a `.cmp_display` method
// Only those pairs for whom T implements both `Display` *and* `PartialOrd`
// can call instance.cmp_display()
impl<T: Display + PartialOrd> Pair<T> {
    // so an instance created with Pair::new(5, 11) could call this method
    // but an instance created with Pair::new("foo", "bar") could not 
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
