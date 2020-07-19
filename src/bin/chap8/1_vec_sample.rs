fn main() {
    // To create a new empty vector, we can call the Vec::new function, pp. 146
    let mut v: Vec<i32> = Vec::new();
    // As with any variable, if we want to be able to change its value,
    // we need to make it mutable using the mut keyword

    // updating a vector, pp. 147
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    let third: i32 = v[2];
    println!("The third element is {}", third);
    match v.get(20) {
        Some(value) => println!("The 20th element is {}", value),
        None => println!("There is no 20th element."),
    }

    let mut v: Vec<i32> = Vec::new();
    populate_vec(&mut v);
    let third: &i32 = &v[3];
    println!("The fourth element is {}", third);
    match &v.get(3) {
        Some(value) => println!("The 4th element is {}", value),
        None => println!("There is no 4th element."),
    }


    // To create a Vec<T> that has initial values,
    // Rust provides the vec! macro for convenience
    let v = vec![1, 2, 3];
    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

fn populate_vec(v: &mut Vec<i32>) {
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}