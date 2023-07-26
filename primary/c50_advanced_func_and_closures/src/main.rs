#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    /* function pointer */
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(|i| i.to_string())
        .collect();
    println!("list_of_strings: {:?}", list_of_strings);
    
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers
        .iter()
        .map(ToString::to_string)
        .collect();
    println!("list_of_strings: {:?}", list_of_strings);

    let list_of_status: Vec<Status> = (0u32..20)
        .map(Status::Value)
        .collect();
    println!("list_of_status: {:?}", list_of_status);

    /* 返回闭包 */
    // fn returns_closure() -> Fn(i32) -> i32 {
    //     |x| x + 1
    // }
    
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }



}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}