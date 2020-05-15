#![allow(dead_code)]

fn value_in_cents(coin: Coin) ->u8{
    match coin{
        Coin::Penny=>{
            println!("Lucky penny!");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}!",state);
            25
        },
    }
}

#[derive(Debug)]
enum Ustate {
    Alabama,
    Alaska,
}

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(Ustate),
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

let some_u8_value=Some(0u8);
if let Some(3)=some_u8_value{
    println!("three");
}

fn main() {
    println!("{:?}", value_in_cents(Coin::Quarter(Ustate::Alaska)));
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);
    println!("{:?},{:?},{:?}",six, none, five);
}