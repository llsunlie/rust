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
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    if let Some(3) = some_u8_value {
        println!("three");
    }

    /* 计数所有不是 25 美分的硬币的同时也报告 25 美分硬币所属的州 */
    let mut count = 0;
    let coin = Coin::Quarter(UsStatus::Alabama);
    match coin {
        Coin::Quarter(state) => println!("Coin::Quarter state is {:?}", state),
        _ => count += 1,
    }
    
    let mut count = 0;
    let coin = Coin::Quarter(UsStatus::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("Coin::Quarter state is {:?}", state);
    } else {
        count += 1;
    }
}
