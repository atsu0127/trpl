// enum
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

impl IpAddrKind {
    fn call(&self) {
        match self {
            IpAddrKind::V6(s) => println!("ipv6: {}", s),
            IpAddrKind::V4(s) => println!("ipv4: {}", s)
        }
    }

    fn func1(&self) {
        println!("func1");
    }
}

// match
enum Coin {
    One(u32),
    Ten(u32),
    Fifty(u32),
    Hundred(u32),
}

impl Coin {
    fn get_value(&self) -> u32 {
        match self {
            Coin::One(count) => 1 * count,
            Coin::Ten(count) => 10 * count,
            Coin::Fifty(count) => 50 * count,
            Coin::Hundred(count) => 100 * count,
        }
    }
}

// optional
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    // enum
    let four = IpAddrKind::V4(String::from("192.168.1.1"));

    four.call();
    four.func1();

    // match
    let ten_yen = Coin::Ten(1);
    println!("ten_yen = {}", ten_yen.get_value());

    let twenty_yen = Coin::Ten(2);
    println!("twenty_yen = {}", twenty_yen.get_value());

    // optional
    let one = Some(1);
    println!("{}", plus_one(one).unwrap_or(10));

    let none = None;
    println!("{}", plus_one(none).unwrap_or(0));

    // if let
    let five = Some(5);
    if let Some(5) = five {
        println!("five!!");
    }

    let four = Some(4);
    if let Some(5) = four {
        println!("five!!");
    } else {
        println!("not five...")
    }
}
