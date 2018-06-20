extern crate tests;

mod common;

#[test]
fn adds_two() {
    for (e,v) in common::adder_data() {
        assert_eq!(e, tests::add_two(v));
    }
}
