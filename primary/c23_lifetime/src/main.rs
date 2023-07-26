use std::fmt::Display;

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    // {
    //     let r;          // ---------+-- 'a
    //                           //          |
    //     {                     //          |
    //         let x = 5;        // -+-- 'b  |
    //         r = &x;           //  |       |
    //     }                     // -+       |
    //                           //          |
    //     println!("r: {}", r); //          |
    // }                         // ---------+

    {
        let x = 5;       // ----------+-- 'b
                              //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    let s1 = String::from("abcd");
    let s2 = "asd";
    let res = longest(s1.as_str(), s2);
    println!("The longest str is: {}", res);

    let s1 = String::from("looooooong");
    {
        let s2 = String::from("xxx");
        let res = longest(s1.as_str(), s2.as_str());
        println!("The longest str is: {}", res);
    }

    /* `s2` does not live long enough */
    // let s1 = String::from("loooooooooong");
    // let res;
    // {
    //     let s2 = String::from("xyz");
    //     res = longest(s1.as_str(), s2.as_str());
    // }
    // println!("The longest str is: {}", res);

    println!("===");

    let novel = String::from("Call me Ishmael. Some years go...");
    let first_sentence = novel.split(".")
        .next()
        .expect("Count not find a '.'");
    let i = ImportantExcerpt { 
        part: first_sentence 
    };

    /* static lifetime */
    let s: &'static str = "I have a static lifetime.";



}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn f1<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

/* lifetime elision */
/* 
第一条规则是每一个是引用的参数都有它自己的生命周期参数。
换句话说就是，
有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，
依此类推。

第二条规则是如果只有一个输入生命周期参数，
那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。

第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章), 
那么所有输出生命周期参数被赋予 self 的生命周期。
第三条规则使得方法更容易读写，因为只需更少的符号。
 */

/* generics, trait bounds, lifetime */
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T: Display {
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}