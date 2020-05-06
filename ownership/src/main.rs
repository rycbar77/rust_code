fn main() {
    // let mut s = String::from("hello");
    // s.push_str(",world!");
    // println!("{}", s);
    //
    // let mut x = 5;
    // let y = x;
    // x = x + 1;
    // println!("{}", x);
    // println!("{}", y);
    let s1 = String::from("hello");
    // let (s2, len) = calculate_length(s1);
    let len2 = calculate_length2(&s1);
    // println!("the length of '{}' is {}.", s2, len);
    println!("the length of '{}' is {}.", s1, len2)
}

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

fn calculate_length2(s: &String) -> usize {
    s.len()
}