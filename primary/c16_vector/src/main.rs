fn main() {
    /* vector init */
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    /* update */
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    {
        let v = vec![1, 2, 3, 4];
    
        // 处理变量 v
    
    } // <- 这里 v 离开作用域并被丢弃

    /* read element */
    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("v[2] = {}", third);

    match v.get(2) {
        Some(third) => println!("v[2] = {}", third),
        None => println!("v[2] = None"),
    }

    /* immutable */
    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = v.get(0);

    // v.push(6); // error

    println!("v[0] = {:?}", first);

    /* for vector */
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{}", i);
    }
    // for i in v {
    //     println!("{}", i);
    // }
    println!("v = {:?}", v);

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    /* enum vector */
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.1),
        SpreadsheetCell::Text(String::from("value")),
    ];
    println!("row = {:?}", row);

    

}
