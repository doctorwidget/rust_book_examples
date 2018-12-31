/**
 * Traits are Rust's interfaces.
 * 
 * Traits are implemented by _structs_.  Rust does not have classes per se, 
 * but structs plus traits gets you all the polymorphic goodness provided by
 * classes, without encouraging the antipattern of deep inheritance.
 * 
 *          (Structs + Traits) > Classes
 * 
 * Traits also count as _Types_, so you can declare them as generic type 
 * modifiers or inside function signatures. For example, you could specify
 * <T: Foo> to mean "any generic type which implements the Foo trait". 
 * Or you could include (x: Foo) in a function signature to say that the `x`
 * argument is "any entity which implements the Foo trait".  
 */

// note that for this demo, we're going to experiment with putting most of
// the code in the one-and-only `lib.rs` singleton.
// The name we `use` here is the one specified in `{root}/Cargo.toml`
use mylib; 

// if we don't *explicitly* bring in the Summary trait, then nothing *related*
// to the Summary trait will be visible inside this scope. That means we need
// to use the trait if we want to use the *methods* from that trait. 
use mylib::Summary;  // now .summarize() is available

// NB: older (pre-2018) rust examples may say you need an extra declaration
// such as `extern crate mylib`, in *addition* to the `use mylib;`
// That was true prior before the Cargo tool became standard. 
// If you're using Cargo, `extern crate foo` statements are no longer required

// As seen in `{root}/Cargo.toml`, this file `foo.rs` is *a* binary. 
// All binary files need *a* main() method.
// This happens to be the *only* binary for this project, so Cargo will infer
// that it is the one-and-only-target when you execute `$: cargo run`
fn main() {
    mylib::greet();

    // create a tweet
    let tweet = mylib::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("a horse is a horse of course of course"),
        reply: false,
        retweet: false,
    };

    let article = mylib::NewsArticle {
        headline: String::from("Man Bites Dog"),
        location: String::from("Albequerque"),
        author: String::from("Fudd, E."),
        content: String::from("Arf Arf"),
    };

    // nb: we had to explicitly `use mylib::Summary` to enable the summarize()
    // method on both structs. This is a good demonstration of the fact that the
    // struct is *independent* of the trait: you can use one without the other!
    println!("1 new tweet: {}", tweet.summarize());
    println!("article: {}", article.summarize());

    // NewsArticle has the default .more() implementation
    println!("Article.more(): {}", article.more());
    // but Tweet has a custom one
    println!("Tweet.more(): {}", tweet.more());

    // demo of both forms of notify
    println!("Demo of 2 variants for using the Summary trait as a first-class type:");
    mylib::notify_a(&article);
    mylib::notify_b(&article);

    // demo of the final and most-elegant version of `largest` from 14_generics:
    let number_list = vec![34, 50, 25, 222, 65];
    let result = mylib::largest(&number_list);
    println!("(generics + traits!) The largest number is {}", result);
    println!("State of number list after the call: {:?}", number_list);

    // nb: interestingly enough, we're still not far from panicky code!
    // just send in an empty vector and our function panics
    // let num_list_2: Vec<i32> = Vec::new(); // perfectly valid empty vector
    // but our largest function makes a brittle reference to T[0]
    // which triggers a panic on an empty vector!
    // let result2 = mylib::largest(&num_list_2);
    // println!("Largest on empty list returns: {}", result2);
    // So really, that method ought to either return a Result<T, E>,
    // or it should *require* a second argument: the value to use when the
    // function is called with an empty vector

    // demo of the Pair struct
    let nums = mylib::Pair::new(5, 11);
    // in this case T is a number, so it qualifies for this conditional method
    nums.cmp_display();
    // it also works with chars, which are of type PartialOrd
    let chars = mylib::Pair::new('a', 'z');
    chars.cmp_display();
    // but not two tweets, which are are neither Display nor PartialOrd
    let tweet2 = mylib::Tweet {
        username: String::from("spam"),
        content: String::from("eggs"),
        reply: false,
        retweet: false,
    };
    let _tweets = mylib::Pair::new(tweet, tweet2); // the _pair_ is valid
    // but this Pair does *not* implement cmp_display!
    // _tweets.cmp_display(); // compiler error! The error message is:
    // `no method named `cmp_display` found for type mylib::Pair<mylib::Tweet>`
}
