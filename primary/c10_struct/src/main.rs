
struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        sign_in_count: 1,
        active: true,
    }
}

/* field init shorthand */
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        sign_in_count: 1,
        active: true,
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("some@example.com"),
        username: String::from("username"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("value");

    /* struct update syntax */
    let user2 = User {
        email: String::from("value"),
        username: String::from("value"),
        sign_in_count: user1.sign_in_count,
        active: user1.active,
    };

    let user2 = User {
        email: String::from("value"),
        username: String::from("value"),
        ..user1
    };
    
    /* tuple struct */
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let color = Color(0, 0, 0);
    let point = Point(0, 0, 0);

    /* unit-like struct */
    // 我们也可以定义一个没有任何字段的结构体！
    // 它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，
    // 即 unit 类型。
    // 类单元结构体常常在你想要在某个类型上实现 trait 但不需要在类型中存储数据的时候发挥作用。
}
