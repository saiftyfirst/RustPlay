// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("Hello");
    s
}