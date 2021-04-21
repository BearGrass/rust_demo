fn main() {
    println!("Hello, world!");

    // stack vs heap
    /* stack
        last in first out
        fixed length, if not will change to heap
        push value to stack do not need malloc
    */
    /* heap
        slow than stack
    */

    /* ower
       folow code and find which code use heap
       minimize same data in heap
       clear data from heap
    */
    /* owner rule
     * every value have a variable, the variable own the value
     * every value only have one owner
     * when owner out of the scope, the value would be delete
     */
    /* scope

    */
    owner_demo();
    string_for_owner_demo();
    owner_demo2();
    reference_demo();
    slice_demo();
}

fn owner_demo() {
    // s can not be use
    //let s = "hello world"; // s can be used
                           // can deal with s
} // the scope of s is end, s can not be used again

fn string_for_owner_demo() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let s1 = s; // s move to s1
    //println!("{}", s);
    let s2 = s1.clone();
    println!("{} {}", s1, s2);

    // data in stack just need copy ,not need clone
    let x = 5;
    let y = x;
    println!("{} {}", x, y); // becase x y in stack
    /*
        u32 bool char double tuple(only element is copy trait) have copy trait
    */
}

fn take_ownership(some_stirng : String) {
    println!("{}", some_stirng);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
}

fn owner_demo2() {
    let s = String::from("hello world");
    take_ownership(s);
    //println!("{}", s);

    let x = 5;
    make_copy(x);

    println!("x : {}", x);

}

fn calculate_length(s :&mut String) -> usize {
    s.push_str(", test");
    s.len()
}

fn reference_demo() {
    let mut s1 = String::from("hello world");
    let len = calculate_length(&mut s1);
    println!("length is {} {}", s1, len);
    {
        let l1 = &s1;
        println!("{}", l1);
    }
    //let mut l1 = &s1;
    //let r = dangling_demo();
}

/*
fn dangling_demo() -> &String {
    let s = String::from("hello");
    &s
}
*/

fn first_word(s : &str) ->&str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn slice_demo() {
    let s = String::from("hello world");
    let word_index = first_word(&s[..]);

    let my_s = "hello world";
    let abc = first_word(my_s);
    //s.clear();
    println!("{} {}", word_index, abc);

    //let test_s = "test world";
}