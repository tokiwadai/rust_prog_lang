fn main() {
    let data = "initial contents";
    let s = data.to_string();

    // use the to_string method,
    // which is available on any type that implements the Display trait,
    // as string literals do, pp. 152
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
}

