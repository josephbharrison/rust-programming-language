#[derive(Debug)]
enum UsState {
    #[allow(dead_code)] 
    Alabama,
    Alaska,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin1 = Coin::Penny;
    let coin2 = Coin::Nickel;
    let coin3 = Coin::Dime;
    let coin4 = Coin::Quarter(UsState::Alaska);

    println!("coin1 is kind: {:?}", coin1);
    println!("coin2 is kind: {:?}", coin2);
    println!("coin3 is kind: {:?}", coin3);
    println!("coin4 is kind: {:?}", coin4);

    let value = value_in_cents(&coin4);
    println!("value of coin4 (cents): {}", value);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
