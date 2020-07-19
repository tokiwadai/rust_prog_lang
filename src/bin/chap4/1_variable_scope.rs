fn main() {
    // 1. string literal, &str
    let s = "hello world!";
    println!("s: {}", s);

    // 2. mutable string literal
    let mut s2 = "hello world!";
    s2 = "hi world!";
    println!("s2: {}", s2);

    // 3. mutable String type, pp. 71
    let mut s3 = String::from("hello");
    s3.push_str(", world!");  // push_str() appends a literal to a String
    println!("s3: {}", s3);          // This will print `hello, world!
}