use std::fmt::Display;


pub trait Summary {
    fn sumarize(&self) -> String;
    fn sumarize_2(&self) -> String {
        String::from("(Read more...)")
    }

    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn sumarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn sumarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.sumarize());
}

/* trait bound */
pub fn notify_1<T: Summary>(item: T) {
    println!("Breaking news! {}", item.sumarize());
}

pub fn notify_2(item: impl Summary + Display) {

}

pub fn notify_3<T: Summary + Display>(item: T) {

}

fn some_function<T: Display + Clone, U: Clone>(t: T, u: U) {
    
}

fn some_function_1<T, U>(t: T, u: U) 
where T: Display + Clone, U: Clone {

}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// return impl trait 只适用于返回单一类型的情况
fn returns_summarizable_2(switch: bool) -> impl Summary {
    // if switch {
        NewsArticle {
            headline: todo!(),
            location: todo!(),
            author: todo!(),
            content: todo!(),
        }
    // } else {
    //     Tweet {
    //         username: todo!(),
    //         content: todo!(),
    //         reply: todo!(),
    //         retweet: todo!(),
    //     }
    // }
}

fn main() {
    let tweet = Tweet {
        username: String::from("asdf"),
        content: String::from("content"),
        reply: false,
        retweet: false,
    };
    let s = tweet.sumarize();
    println!("tweet = {}", s);

    /* default trait */
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("New article available! {}", article.sumarize_2());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    println!("===");

    /* 使用 trait bound 有条件地实现方法 */
    let t = Pair::new(1, 3);
    t.cmp_display();

    let s = 3.to_string();


}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Self { x: x, y: y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest num is {}", self.x);
        } else {
            println!("The largest num is {}", self.y);
        }
    }
}

/* 对任何实现了特定 trait 的类型有条件地实现 trait。 */
/* blanket implementations */

pub trait AnoTrait {
    fn f() -> String;
}

impl AnoTrait for dyn Summary {
    fn f() -> String {
        todo!()
    }
}