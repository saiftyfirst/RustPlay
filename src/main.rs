// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    let mut s = String::from("hello world");

    let first = get_word(&s);

    println!("The first word is: {}", first);

    s.clear();

}

fn get_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
