
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    ip_type: IpAddrKind,
    address: String,
}

enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Ipv4Addr {
    // -- snip --
}

struct Ipv6Addr {
    // -- snip --
}

enum std_IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        ip_type: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        ip_type: IpAddrKind::V6,
        address: String::from("::1"),
    };

    /* enum + type */
    let home = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback = IpAddr2::V6(String::from("::1"));

    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    /* another example */
    let m = Message::Write(String::from("hello"));
    m.call();

    /* Option */
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = Some(5);
    // let sum = x + y; // error


}

fn route(ip_type: IpAddrKind) {}