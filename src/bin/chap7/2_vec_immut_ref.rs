fn main() {
    // 1. Immutable & mutable in the same scope, pp. 149
    // Rule: you can’t have mutable and immutable references in the same scope, pp. 85
    let mut v = vec![1, 2, 3, 4, 5];
    // we hold an immutable reference to the 1st element in a vector
    let first = &v[0];

    // and try to add an element to the end, which won’t work.
//    v.push(6);
    println!("The first element is: {}", first);


    // 2. Multiple mutable references in the same scope,
    // Rule: you can’t have multiple mutable references in the same scope., pp. 83
    let mut v = vec![1, 2, 3, 4, 5];
    //first mutable borrow occurs here
    let first = &mut v[0];
    // second mutable borrow occurs here
    v.push(6);
    println!("The first element is: {}", first);
}

