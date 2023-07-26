fn main() {
    let v1 = vec![1, 2, 3];
    /* 迭代器是 惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果 */
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

pub trait Iterator {
    type item;

    fn next(&mut self) -> Option<Self::item>;
}