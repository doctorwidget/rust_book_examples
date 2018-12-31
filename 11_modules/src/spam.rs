/**
 * Module file defined all-in-one as a peer. It is used by main.js as the
 * `spam` module, which means the file itself must be named `spam.rs`.
 * 
 * *But* this file does *not* need to declare _itself_ as the spam module
 * internally. Our module name is implicitly based on our file name, and it is
 * declared explicitly by the _user_ of this file: main.js. Then, within this 
 * file, we don't have to redundantly re-declare `mod spam`. In fact, if we did
 * that, we would be declaring a *sub* module!  
 * 
 * (NB: this is different from many other languages, which require repeated 
 * explicit, redundant and consistent redeclarations. E.g. in some languages, 
 * the full file path must be accompanied by a matching declaration within 
 * the file itself. Not so in Rust!)
 */

// As mentioned above, if we *do* use `mod` internally, it creates a submodule.
pub mod eggs {
    pub fn toast(path: &str) {
        println!("I am Toast; hear me roar (via a {} path!)", path);
    }

    // this function demonstrates the use of `super::` 
    // this is the only way to reach *up* and *over*
    pub fn ham() {
        // without super::, ham cannot see up to beans
        // beans("ham cannot see beans directly"); 
        // compiler error > beans not found in this scope
        
        // but this works:
        super::beans("message from ham");
    }
}

// this fn is part of the spam module
pub fn beans(msg: &str)  {
    println!("Beans says: {}", msg);
}

