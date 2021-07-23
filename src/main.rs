// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;

use std::ops::Add;
use std::fmt;

struct User {
    username: String,
    email:  String,
    sign_in_count: u64,
    active: bool
}

struct Color(i32, i32, i32);

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Color {
        Color((self.0+rhs.0)/2, (self.0+rhs.0)/2, (self.0+rhs.0)/2)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.0, self.1, self.2)
    }
}

impl std::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

fn main() {

    let current_user  = User {
        email: String::from("ahmedsaif.developer@gmail.com"),
        username: String::from("saiftyfirst"),
        active: true,
        sign_in_count: 1
    };

    // current_user.email = String::from("new_email@proton.com");

    println!("Current username: {}", current_user.username);


    let mut mutable_user  = User {
        email: String::from("ahmedsaif.developer@gmail.com"),
        username: String::from("saiftyfirst"),
        active: true,
        sign_in_count: 1
    };

    mutable_user.email = String::from("new_email@proton.com");
    println!("New email: {}", mutable_user.email);

    let user3 = User {
        email: String::from("user3@gmail.com"),
        ..mutable_user
    };

    let se = String::from("se");
    let su = String::from("su");
    let u2 = build_user(se.clone(), su);

    println!("First: {}", u2.username);
    println!("Second: {}", se);

    let black = Color(0, 0, 0);
    let white = Color(256, 256, 256);

    let grey = Color(128, 128, 128);

    let mix = black + white;
    println!("{}", mix);

    println!("{}", mix == grey);


}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}


