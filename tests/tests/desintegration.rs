mod common;

#[test]
fn test_data_valid() {
    for (e,v) in common::adder_data() {
        assert_eq!(e - 2, v);
    }
}
