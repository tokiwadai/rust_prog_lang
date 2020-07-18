#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("Coin: {:?}", value_in_cents(Coin::Penny));
    println!("Coin: {:?}", value_in_cents(Coin::Nickel));
    println!("Coin: {:?}", value_in_cents(Coin::Dime));
    println!("Coin: {:?}", value_in_cents(Coin::Quarter));
}
