fn main() {
    // 1. Stack-Only Data: Copy
    let x = 5;
    let y = x;
    println!("x: {}, y; {}", x, y);

    // 2. double free error
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1: {}, s2: {}", s1, s2);  // error since s1 is out of scope already
    println!("s2: {}", s2); // s1 moved to s2

    // 3. Clone - deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();    // deep copy, EXPENSIVE!
    println!("s1 = {}, s2 = {}", s1, s2);
}