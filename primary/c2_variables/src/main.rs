
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);
    // shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is {}", x);
    let space = "    ";
    let space = space.len();
    

}
