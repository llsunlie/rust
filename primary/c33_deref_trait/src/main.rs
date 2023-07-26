use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    /* dereference operator */
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /* use Box<T> like dereference */
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /* use MyBox<T> */
    // deref 方法返回值的引用，以及 *(y.deref()) 括号外边的普通解引用仍为必须的原因在于所有权。
    // 如果 deref 方法直接返回值而不是值的引用，其值（的所有权）将被移出 self。
    // 在这里以及大部分使用解引用运算符的情况下我们并不希望获取 MyBox<T> 内部值的所有权。
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    /* deref coercions */
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&(*m)[..]);

    


}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}