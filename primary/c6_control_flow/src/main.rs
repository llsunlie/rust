fn main() {
    /* if */
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    println!("======================================");

    // if number {
    //     println!("number was three");
    // }

    /* else if */
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("========================================");

    /* let if */
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    // let condition = true;
    // let number = if condition {
    //     5
    // } else {
    //     "six"
    // }
    // println!("The value of number is: {}", number);

    /* loop */

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("========================================");

    /* while */
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    println!("========================================");

    /* for */
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The element value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}