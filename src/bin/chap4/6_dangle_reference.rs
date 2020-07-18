fn main() {
    // 1. Dangle reference
    // ERROR
//    let reference_to_nothing = dangle();

    // 2. Non Dangle reference
    let reference = no_dangle();
    println!("reference: {}", reference)
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!


fn no_dangle() -> String {
    let s = String::from("hello");
    s
}