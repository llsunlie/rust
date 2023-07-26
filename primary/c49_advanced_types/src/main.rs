use std::io::Error;
use std::fmt::{self, Arguments};
use std::io::Result;

pub trait Write {
    // type Result<T> = std::result::Result<T, std::io::Error>;
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()>;
}

fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

fn main() {

    /* 类型别名用来创建类型同义词 */
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);


    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // fn returns_long_type() -> Thunk {
    //     // --snip--
    //     Thunk {}
    // }


}

/* 从不返回的 never type */
// fn bar() -> ! {
//     // --snip--
// }