fn main() {
    /* first word */
    let mut s = String::from("hello world");

    let word = first_word(&s);
    
    s.clear();

    println!("word = {}", word);

    /* slice */
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2];

    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    let slice = &s[0..len];
    let silce = &s[..];

    /* new first word */
    let mut s = String::from("hello world");
    
    let word = first_word2(&s);

    // s.clear(); // error

    println!("the first word is {}", word);

    /* new new first word */
    let s1 = String::from("hello world");

    let word = first_word3(&s1[..]);
    
    let s2 = "hello world";
    
    let word = first_word3(&s2[..]);
    let word = first_word3(s2);

    /* other slice */
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}