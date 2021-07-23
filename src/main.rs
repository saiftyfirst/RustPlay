// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {

    let x = plus_one(5);
    println!("Value of x: {}", x);

}

fn plus_one(x: i32) -> i32 {
    x + 1 // Adding a semicolon means that the expression value is not returned
}
