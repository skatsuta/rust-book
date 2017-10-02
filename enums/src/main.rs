#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?} called", self)
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopbck = IpAddr::V6(String::from("::1"));
    println!("home: {:?}, loopback: {:?}", home, loopbck);

    let m = Message::Write(String::from("hello"));
    m.call();

    let coin = Coin::Penny;
    println!("{} cents", value_in_cents(coin));
}
