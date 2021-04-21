//const MAX_POINT:u32 = 100_000;
fn main() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);
    println!("Hello, world!");
    let x = 5;
    println!("The value is {}", x);

    //x = 6;
    let x = x + 1;
    let x = x * 2;
    println!("{}", x);

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);

    number();
    multi();
}

fn number() {
    println!("start number ");
    let mut a: u8 = b'A';
    println!("{}", a);
    a = 255;
    println!("{}", a);

    let z = '😁';
    println!("{}", z);
}

fn multi() {
    // tuple
    // any type and length fixed
    let tup: (i32, f64, u8) = (500, 6.3, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // array one type and length fixed
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [u32; 5] = [3; 5];

    // vector one type and length can append
    //test
}
