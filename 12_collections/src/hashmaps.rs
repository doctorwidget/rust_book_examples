/**
 * HashMaps _are_ part of the standard library, hooray!
 * 
 * But they are _not_ part of the prelude, so you have to explicitly iport
 * them, wtf! I take great umbrage at this: hashmaps are the single greatest
 * data structure known to humanity, and they deserve better. 
 */
use std::collections::HashMap;

// demo string-related code 
pub fn demo_hashmaps () {
    let divider = "///////////";

    // HashMaps are Rusts' canonical dictionary implementation
    
    println!("{}", &divider);
    println!("--- HashMap Demonstration Begins --- ");

    let mut scores1 = HashMap::new();

    scores1.insert(String::from("Blue"), 101);
    scores1.insert(String::from("Red"), 50);
    println!("Scores: {:?}", scores1);
    
    // iterate over keys and values in the map
    println!("Scores1 keys and values:");
    for (key, value) in &scores1 {
      println!("... {}: {}", key, value);
    }

    // here's an alternative initialization strategy, which is so verbose
    // and fugly that I am amazed they included it!
    let teams  = vec!["Green".to_string(), "Yellow".to_string()];
    let _scores = vec![42, 14];
    // now the verbosity happens!
    let scores2: HashMap<_, _> = teams.iter().zip(_scores.iter()).collect();
    println!("Scores2: {:?}", scores2);

    // access using `.get` with a _borrowed_ key reference
    let green_key = String::from("Green");
    // but you get Option<i32> back, not plain old <i32>, eek!
    let green_score = scores2.get(&green_key); // because the value might not exist!
    match green_score {
      Some(score) => println!("Green score: {}", score),
      None => println!("Green has no score")
    }
    // confirm that nothing was consumed and no ownership changed
    println!("Scores2: {:?}", scores2);

    // Setting and inserting a value are both done via .insert()
    let mut colors = HashMap::new();
    colors.insert(String::from("blue"), 400);
    colors.insert(String::from("red"), 700);
    // but wait, 400 nanometers is indigo, not blue!
    colors.insert(String::from("blue"), 456); // that's more like it
    println!("Colors and wavelengths: {:?}", colors);

    // use .entry() to insert only if the key was unset before
    // then add a .or_insert() clause for the value to insert if so
    // So this first call will not change the blue entry at all, and call no errors either
    colors.entry(String::from("blue")).or_insert(444);
    // but this call _will_ insert a new entry
    colors.entry(String::from("yellow")).or_insert(555);
    println!("Colors and wavelengths: {:?}", colors);

    // NB:  the .or_insert() clause returns a mutable borrow for the value,
    // so you can be more sophisticated about how you update it.
    let text = "hello world wonderful world";

    let mut words = HashMap::new();

    for word in text.split_whitespace() {
      let count = words.entry(word).or_insert(0);
      // count is of type &mut V, where V is the value type
      // so we have to dereference it and update it in place
      *count += 1;
    }

    println!("word map for '{}': {:?}", text, words);

    println!("--- HashMap Demonstration Finish --- ");
    println!("{}", &divider)
}