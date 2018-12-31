/**
 * __Ownership__ is the central innovation of rust memory management.
 * 
 * Other languages routinely let you share cheap *references* to entities in
 * memory. This is an incredibly powerful tool for allowing different parts of 
 * the  program operate on one shared data structure. Combined with mutability
 * by default, this lets programmers play all kinds of magic tricks. But 
 * unfortunately, it turns out that most humans can't be trusted with that kind 
 * of power, and they will inevitably create all sorts of errors once they start 
 * down this road. Spooky action at a distance is not your friend!
 * 
 * Rust gets away from that by making a strong committment to the concept
 * that every entity in memory has one and only one owner at a time. You 
 * cannot even make a second variable that *refers* to the first one without
 * *automagically* transferring ownership of that entity!
 * 
 *    let foo = String::from("Hello, world");
 *   // the foo var is the one-and-only owner of this string
 * 
 *    let bar = foo; // ownership is *transferred* by default! WTF?!?   
 *    // the term of art for this transfer of ownership is `move`
 *    // after the move, now *bar* is the owner, and foo owns *nothing*
 *    // THERE CAN BE ONLY ONE.
 * 
 *    println!("{}", foo); // compiler error: 'value borrowed here after move'
 * 
 * Make no mistake, this is a *radical* departure from the pattern seen in
 * every other language I have ever used. This pattern is why Rust gives you
 * memory safety *without* a garbage collector. It is also why Rust is simply 
 * *immune* to  whole categories of errors that can exist in other languages. 
 * 
 * This concept of *strong ownership* has great synergy with a preference for
 * *immutability*, which is another radical concept that won you over despite 
 * seeming completely impractical at first glance. You need ubiquitous access 
 * to low-level language constructs designed to work with this approach for
 * it to work in practice. Rust offers you those tools for strong ownership,
 * just as Clojure does for immutability. 
 * 
 * Given the behavior demonstrated above, you obviously don't want to go around 
 * transferring ownership willy-nilly. Thus the `foo-bar` example above would 
 * never happen in actual Rust code  (despite being a run-of-the-mill remedial 
 * kind of example in other languages).
 * 
 * Instead, you will think more carefully about who is your one-and-only owner.
 * You will only transfer ownership intentionally, and at the utmost need. 
 * Most of the time, other variables will be given special limited *read only* 
 * access to that value. If necessary, you can give out a special *read+write* 
 * permission for that value. The one and only owner can do either of those 
 * things without losing ownership -- but never both at the same time!
 * 
 * In other words, you can give out:
 * 
 * - any number of read-only references (aka "borrows"), using a `&` ref
 * - one and only one *read+write* reference, using a `&mut` ref
 * 
 * But you can never cross those streams! It's 1..N read-only within the
 * current scope, *OR* one (1) read+write in the current scope. The compiler
 * will never let you mix and match the two: it will diligently perform a 
 * comprehensive search of the entire tree of all possible scopes created by
 * all possible code paths, to make sure the streams never cross. Experienced 
 * programmers from other languages are often surprised and frustrated by how 
 * *easy* it is to cross the streams without realizing it. The term of art for 
 * their frustration is "fighting the borrow checker".
 * 
 * So this is our starting point, from which everything unique about Rust
 * will follow.
 * 
 * 1. THERE CAN BE ONLY ONE (owner)
 * 2. Naively pointing to the value of a variable _takes ownership_ by default
 * 3. You can give out any number of `read-only` references using `&`
 * 4. You can give out one (1) `read+write` reference using `&mut`
 * 5. You can never ever mix and match 3 and 4 in the same scope
 * 
 */
fn main() {
    let foo = String::from("Hello, world");
    let bar = foo;
    //println!("{}", foo); // compiler error!
    println!("String owned by bar: {}", bar);

    // Create a read-only reference (aka a *BORROW*) like so
    let zug = &bar; // the magic is in the `&`
    println!("String *borrowed* from bar: {}", zug);
    // there has been no *MOVE*, only a *BORROW*
    println!("Bar is still the owner: {}", bar);

    // you can create multiple borrows within one scope:
    let qux = &bar; // another borrow... *wheeeee! I am saving memory!*
    println!("String *borrowed* from bar twice in one scope: {}", qux);
    println!("Yet bar is still the owner: {}, bar", bar);

    // Since we have at least one read-only reference in this scope,
    // any attempt to create a `read+write` reference is a compiler error
    // let ack = &mut bar; // compiler error > (see next line)
    // > 'cannot borrow 'bar' as mutable because it is also borrowed as immutable'
    // println!("Three refs at once: {}, {}, and {}", zug, qux, ack);

    // Hence you would have to intentionally *clone*, which is a good 
    // defensive habit when dealing with mutable data anyway
    let mut clone = bar.clone(); // clone is the only owner of this new copy
    
    fn mutate(target: &mut String) {
        target.push_str("... mutated! ...");
        // above is a statement, not an expression, so this returns nothing

        // but that's fine: we are demonstrating mutability, not idempotence.

        // Whoever calls this function retains ownership of the `target`
    }   
    
    let ack = &mut clone; // this is our one-and-only-one allowed mutable borrow
    mutate(ack); // change clone via spooky action at a distance
    println!("clone modified via mutable borrow: {}", clone);
    
    // But trying to use a second mutable reference is a no go!
    // let wtf = &mut clone;
    // println!("second mutable borrow not allowed: {}, {}", clone, wtf);
    // > `cannot borrow clone as immutable because it also borrowed as mutable'

    // Note that you can often create crossed-streams refs, as long as you
    // never try to transfer them out of the current scope
    let murb = &clone; // immutable murb in same scope as mutable ack
    println!("Murb var is {} long", murb.len()); // no error here!
    // The compiler only complains when you try to send them out to an
    // external scope, as in this next line:
    // println!("Mutable and immutable borrows cannot coexist: {}, {}", ack, murb);

    // in practice, I found the borrow checker *less* picky than I was
    // expecting to, after hearing so many complaints about it in advance. 
    // Inside one big main() fn like this, you can often "cross the streams",
    // as we do with ack and murb above. So for very simple scenarios, it does
    // not seem like the borrow checker is going to be some kind of huge PITA.


    // Finally, note that you _can_ use more-familiar patterns for primitives
    let x = "blargh";
    let y = x; // this does *not* trigger a Move, and there is no need for `&`
    let xy = [x, y];
    println!("Primitive values have the Copy trait: {:?}", xy);
    // It turns out there is a trait called "Copy" that very primitive values
    // have, which says "I am cheap to copy, so make copies of me willy-nilly".
    // In practice, this is only true for numbers, strings, booleans, etc.
    // You could also add Copy to entities that you define yourself, if needed.
    // Entities with the Copy trait work like immutable primitive data does in
    // other languages, even though technically they are getting copied, and 
    // not reused (as in, for example, Clojure)
}
