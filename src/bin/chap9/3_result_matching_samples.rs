use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    // enum io::ErrorKind is provided by the standard library
    // and has variants representing the di erent kinds of errors
    // that might result from an io operation.
    // The variant we want to use is ErrorKind::NotFound,
    // which indicates the  le we’re trying to open doesn’t exist yet, pp. 175
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    println!("{:?}", f);

    let f = File::open("hello1.txt")
            .unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt")
                    .unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);

    // unwrap is a shortcut method that is implemented just like the match expression.
    // If the Result value is the Ok variant, will return the value inside the Ok .
    // If the is the Err variant, unwrap will call the macro for us, pp. 176
//    let f = File::open("hello2.txt").unwrap();

    let f = File::open("hello2.txt")
        .expect("Failed to open hello2.txt");
}