#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let s = Rectangle::square(20);
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    let rect1 = Rectangle {
        width: 20,
        length: 40,
    };

    println!("{:#?} {} {}", rect, rect.area(), rect.can_hold(&rect1));
}
