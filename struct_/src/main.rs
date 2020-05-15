use std::net::Shutdown::Read;

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32
    {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("hhhhhh"),
        username: String::from("ssssss"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another");

    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:#?}", rect1);
    println!("the area of the rectangle is {} square pixels.",
             rect1.area());

    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
