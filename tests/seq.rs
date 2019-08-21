extern crate comb;
use comb::seq;

#[test]
fn seq_test() {
    assert_eq!(seq("test").parse("testing"), Ok("test".to_string()));
    assert_eq!(seq("test").parse("tesing"), Err(()));
}
