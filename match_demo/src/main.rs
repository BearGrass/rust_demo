fn main() {
    println!("Hello, world!");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let v: u8 = 0;

    let v = Some(3u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }
    if let Some(3) = v {
        println!("three");
    } else {
        println!("none");
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
