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

    scalar();
    compound();
    function_demo(5, 6); // argument
}

fn scalar() {
    println!("start number ");
    let mut a: u8 = b'A';
    println!("{}", a);
    a = 255;
    println!("{}", a);

    let z = '😁';
    println!("{}", z);
}

fn compound() {
    // tuple
    // any type and the length is fixed
    let tup: (i32, f64, u8) = (500, 6.3, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);

    // array one type and the length  is fixed
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b: [u32; 5] = [3; 5];
    println!("{}, {}", a[4], b[3]);
    //let c = [5, 6];
    //println!("{}", a[c[0]]);

    //let month = ["monday", "truesday", "wendsday", "saturday", "friday"];
}

fn function_demo(x: i32, y: i32) {
    // parameter
    println!("start func demo");
    println!("parameter 1 {}, para 2 {}", x, y);

    let z = {
        let x = 1;
        //x + 3; //()
        x + 3
    };
    println!("z is {}", z);
    println!("five : {}", five_plus(1))
}

fn five_plus(x: i32) -> i32 {
    5 + x
}
