

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Option<T> {
    None,
    Some(T),
}

// fn route(ip_kind: IpAddrKind) {

// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Hello, world! Enums | Match | Control Flow");
    
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y = Some(5);

    let sum = x + y.unwrap();


    println!("{:?} {:?} {}",home,loopback,sum)
}
