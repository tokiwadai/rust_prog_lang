// Reference is BORROWING, pp. 81
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // s1 still valid, since it was just borrowed when passed to calculate_length
    println!("The length of '{}' is {}.", s1, len);


    // #2 Immutable References, does NOT work!!
    let s1 = String::from("hello");
    change_immut(&s1);

    // #3 Mutable References
    let mut s1 = String::from("hello");
    change_mut(&mut s1);
    println!("s1: {}", s1)
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of
  // what it refers to, nothing happens.

fn change_immut(some_string: &String) {
//    some_string.push_str(", world");  // ERROR, trying to modify immutable references
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}