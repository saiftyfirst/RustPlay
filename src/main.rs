
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {

    let rect_one = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The rectangle is {:?}",
        rect_one
    );
}

fn area(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

