// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {

    for number in 1..4 {
        println!("{}", number);
    }

    for number in (1..5).rev() {
        println!("{}", number);
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter().rev() {
        println!("this value is: {}", element);
    }


}
