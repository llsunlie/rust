fn main() {
    /* borrowing */
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of s1: {}", len);

    /* modify borrowing */
    let s = String::from("hello");

    change(&s);

    /* modify mut borrowing */
    let mut s = String::from("hello");

    change(&mut s);

    /* only one mut reference */
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);

    /* another1 */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    /* another2 */
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;

    println!("{}, {} and {}", r1, r2, r3);

    /* another3 */
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    /* dangling references */
    let reference_to_nothing = dangle();

    let reference = no_dangle();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}