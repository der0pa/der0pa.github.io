#![allow(unused_variables)]
    
fn main() {


enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

let home = IpAddr::V4(127,0,0, 1);
let loopback = IpAddr::V6(String::from("::1"));

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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None); 

} // close main

