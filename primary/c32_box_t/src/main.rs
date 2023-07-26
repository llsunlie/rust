use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    /* Box<T> */
    let b = Box::new(5);
    println!("b = {}", b);

    /* Cons */
    let list = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil))
            ))
        ));

    

}
