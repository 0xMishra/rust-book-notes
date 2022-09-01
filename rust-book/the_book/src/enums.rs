// , enums give you a way of saying a value is one of a possible set of values.

// defining an enum
enum IpAddr {
    V4(u8, u8, u8, u8), // different variant can store different set and type of data
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// we can aslo define methods on enum just like structs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
pub fn run_enums() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
    route(IpAddr::V6(String::from("127.0.0.1"))); // enum passed as an argument
    let m = Message::Write(String::from("hello"));
    m.call();
}

// enums as function parameter
fn route(ip_kind: IpAddr) {}

// the option enum by standard library
enum Option<T> {
    None,
    Some(T),
}
