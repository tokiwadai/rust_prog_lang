fn main() {
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("if let three");
    }

    // if let else
    let some_u8_value = Some(4);
    if let Some(3) = some_u8_value {
        println!("if let three");
    } else {
        println!("if let NOT three");
    }

}
