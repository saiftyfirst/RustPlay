// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    println!("The value of x is: {}", tup.0);


    // arrays
    let months = ["January", "February", "March", "April"];
    println!("2nd month is: {}", months[1]);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Length of array is: {}", a.len());

    let b: [i32; 5] = [3; 5];
    println!("Length of array is: {}", b.len());
    let mut i = 0;
    loop {
        match i < b.len() {
            true => {
                println!("Element i: {}", b[i]);
                i += 1;
            }
            false => break
        }
    }




}
