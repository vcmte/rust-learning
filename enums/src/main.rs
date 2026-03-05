/* Wrong way: set of structs instead of enum usage
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body
    }
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddr {
    fn format(&self) -> String {
        match self {
            IpAddr::V4(a, b, c, d) => format!("{a}.{b}.{c}.{d}"),
            IpAddr::V6(addr) => addr.clone(),
        }
    }
}

fn main() {
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(&localhost);
    route(&loopback);

    // ######################################################

    let buf = Message::Write(String::from("hello"));
    dbg!(&buf);
    dbg!(&buf.call());

    // ######################################################

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    println!("{some_number:?}, {some_char:?}, {absent_number:?}");

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    println!("{x} + {y:?} = ?");
    // let sum = x + y; // error cuz x and y doesn't have the same type
}

fn route(ip_address: &IpAddr) {
    println!("{}", ip_address.format());
}
