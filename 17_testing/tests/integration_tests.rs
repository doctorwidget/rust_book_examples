/**
 * A sample integration test file. 
 * 
 * There's nothing special about the *name* of this file, but the *location*
 * is highly significant by convention. EVERY file inside the `{root}/tests` 
 * directory is treated as a test file. All the code in that directory gets
 * an __implicit__ `#[cfg(test)]` annotation automagically. 
 * 
 * Thus none of this code is included in a regular compilation, and it's 
 * not available for reuse in other files. It is strictly test-only.
 * 
 * You can place as many files as you like in the `{root}/tests/` directory.
 * The more the merrier!
 * 
 * To home in on a specific integration test *file*, use the `--test` flag:
 * 
 *    cargo test --test integration_tests
 * 
 * That should run all the tests in this file, and *only* the tests in this file. 
 * 
 */
use mylib;

#[test]
fn integration_experiment() {
    // this should be included when you run *all* tests
    // or when you run `cargo test integration`
    // or when you run `cargo test ex`
    assert_eq!(4, mylib::add_two(2));
}
