// Listing 11-13: An integration test of a function in
// the adder crate
use adder::add_two;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, add_two(2));
}
