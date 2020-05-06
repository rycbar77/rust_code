fn main() {
    let a=another_function(5, 6);
    if a {
        println!("hello world");
    }

}

fn another_function(x: i32, y: i32) -> bool {
    println!("the value of x is: {}", x);
    println!("the value of y is: {}", y);
    return true;
}
