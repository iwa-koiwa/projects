#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}",state);
            25
        },
    }
}

fn main(){
    let coin = Coin::Quarter(UsState::Alabama);
    let coin2 = Coin::Dime;
    println!("value in cents:{}",value_in_cents(coin2));
    println!("value in cents:{}",value_in_cents(coin));
}