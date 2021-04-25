#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {}
}

fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        addr: String::from("::1"),
    };

    println!("{:?} {:?}", home, loopback);

    let q = Message::Quit;
    let m = Message::Move { x: 12, y: 24 };
    let w = Message::Write(String::from("hello"));
    let c = Message::ChangeColor(1, 2, 3);

    c.call();

    let some_number = Some(5);
    let some_string = Some("abc");
    let absent_number: Option<i32> = None;
    let x = 1 + some_number.unwrap();
    println!("{}", x);

    let x = Some(15);
    let y = x.map_or(false, |s| s == 12);
    print!("y = {}", y);
}
