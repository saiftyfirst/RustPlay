/* ENUMS v1
enum IpAddrKind {
    V4,
    V6
}
// ENUMS v1
struct IpAddr {
    kind: IpAddrKind,
    address: String
}
*/

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

// struct Ipv4Addr {
//     // --snip--
// }
//
// struct Ipv6Addr {
//     // --snip--
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
    }
}

fn main() {

    /*    ENUMS v1
    let home = IpAddrKind {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddrKind {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };*/

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

}


