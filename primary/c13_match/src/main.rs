#[derive(Debug)]
enum UsStatus {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsStatus),
}

fn main() {
    let coin = Coin::Quarter(UsStatus::Alabama);
    value_in_cents(coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    /* match _ */
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("1"),
        3 => println!("3"),
        5 => println!("5"),
        _ => println!("not 1, 3, 5"),
    }
    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Coin::Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Coin::Quarter state: {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}