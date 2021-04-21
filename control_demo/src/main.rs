fn main() {
    if_demo();
    loop_demo();
}

fn if_demo() {
    println!("========= start if demo =========");
    // sequential execution
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else if number == 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number is {}", number);
}

fn loop_demo() {
    println!("========= start loop demo =========");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("the result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("while loop end");

    let a = [10, 20, 30];
    let mut index = 0;
    while index < 3 {
        println!("value is {}", a[index]);
        index += 1;
    }

    // optimize loop array
    let a = [10, 20, 30];
    for ele in a.iter() {
        println!("value is {}", ele);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
