struct User {
    user_name: String,
    email: String,
    sing_in: u64,
    active: bool,
}

fn new_user(email: String, user_name: String, sing_in: u64, active: bool) -> User {
    User {
        email,
        user_name,
        sing_in,
        active,
    }
}


fn main() {
    let mut user1 = new_user(String::from("a@test.com"),
    String::from("a"), 1, false);
    user1.email = String::from("abc@test.com");
    println!("{}", user1.email);

    let user2 = User {
        ..user1
    };
    println!("{}", user2.active);
}
