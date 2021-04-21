use std::io;
use rand::Rng;


fn main() {
    println!("guessing game");
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("secret number is {}", secret_number);
    loop {
        println!("guess one number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("can't read line");
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("your guessing number is {}", guess);
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Greater => println!("Too big"),
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            },
        }
    }
}
