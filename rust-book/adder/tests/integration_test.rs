use adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(adder::add_2(2), 4);
}
