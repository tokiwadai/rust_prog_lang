fn main() {
    // 1. use a for loop to get immutable references to each element
    // in a vector of i32 values and print them, pp. 150
    let v = vec![100, 32, 57];
    for i in &v {
        println!("i: {}", i);
    }

    // 2.  iterate over mutable references to each element, pp. 150
    // in a mutable vector in order to make changes to all the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable reference refers to,
        // we have to use the dereference operator ( * )
        // to get to the value in i before we can use the += operator.
        *i += 50;
    }

    for i in &v {
        println!("i: {}", i);
    }


    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("SpreadsheetCell: {:?}", i);
    }

}

