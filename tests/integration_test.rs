extern crate sample;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, sample::add_two(2));
}
