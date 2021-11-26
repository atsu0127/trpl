mod common;

extern crate test_structure;

#[test]
fn is_adds_two() {
    common::setup();
    assert_eq!(4, test_structure::add_two(2));
}