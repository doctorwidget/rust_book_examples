/**
 * This is the definition for the 'sound' module.  
 * Other rust files refer to it when they include the following:
 * 
 *    mod sound; 
 * 
 * Ending with a semicolon instead of a block {} tells the compiler
 * to go find `./sound.rs`... that's this file!
 * 
 * We do not need to re-declare that we are the 'sound' module: that happens
 * automatically by virtue of this file being named `sound.rs`. In fact, if
 * we were to put a `mod sound` in this file, we would be declaring an inner
 * _submodule_.
 * 
 * tl;dr: you never re-declare your file name as the local module name!
 */

// and here we do just what we mentioned above: create a _submodule_
pub mod instrument; 
// Which kicks off a similar chain. Except it's not really the same as it
// was inside `main.js` at all! We don't find any _peer_ file with the name
// `instrument.js`... but the compiler does find and use the file found at
//`sound/instrument.js`. But why does it know to do that? 

// There's a troubling logical leap here by the compiler that I'm not seeing 
// discussed in the documentation: if you can't find a peer file, do you 
// _always_ get a second chance to find a similar file within a peer directory 
// with your own name?  Could the singleton  `.../src/main.rs` make declare a 
// `mod zug;`, which could then be found at ".../src/main/zug.rs"? This seems
// to be exactly what `sound.rs` is doing here. 
 
 