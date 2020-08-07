use rust_prog_lang::front_of_house::adder;
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn it_adds_two_too() {
    assert_eq!(6, adder::add_two(4));
}
