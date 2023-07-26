fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    /* different contents */
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    /* update String */
    let mut s = String::from("hello");
    s.push_str(", world");

    let mut s1 = String::from("hello");
    let s2 = ", world";
    s1.push_str(s2);
    println!("s2 = {}", s2);

    s1.push('.');
    println!("s1 = {}", s1);

    /* + and format! */
    let s1 = String::from("hello");
    let s2 = String::from(", world");
    let s3 = s1 + &s2;
    println!("s3 = {}", s3);
    println!("s2 = {}", s2);

    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s = {}", s);

    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from("s3");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s = {}", s);
    
    /* string index */
    let s1 = String::from("hello");
    // let h = s1[0]; // error

    let len = String::from("Hola").len();
    println!("len = {}", len);
    let len = String::from("Здравствуйте").len();
    println!("len = {}", len);
    let hello = "Здравствуйте";
    // let answer = &hello[0]; // error

    /* slice */
    let hello = "Здравствуйте";
    let slice1 = &hello[0..4];
    println!("silce1 = {}", slice1);
    // let slice2 = &hello[0..1]; // error
    // println!("silce2 = {}", slice2);
    
    /* for string */
    let hello = "Здравствуйте";
    for (i, item) in hello.chars().enumerate() {
        println!("s[{}] = {}", i, item);
    }
    let v = vec![1, 2, 3];
    for (i, item) in v.iter().enumerate() {
        println!("v[{}] = {}", i, item);
    }
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    let s = "йтеABCनमस्ते";
    for (i, item) in s.chars().enumerate() {
        println!("s[{}] = {}", i, item);
    }


}
