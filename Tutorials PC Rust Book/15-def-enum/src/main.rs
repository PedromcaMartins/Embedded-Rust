enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrMoreCorrect {
    V4(String),
    V6(String),
}

enum IpAddrEvenMoreCorrect {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddrEvenEvenMoreCorrect {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("Hello, world!");

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrMoreCorrect::V4(String::from("127.0.0.1"));

    let loopback = IpAddrMoreCorrect::V6(String::from("::1"));

    let home = IpAddrEvenMoreCorrect::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    m.call();


    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; - Err. y can be None
}

fn route(ip_kind: IpAddrKind) {}
