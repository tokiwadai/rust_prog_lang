use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let s = read_username_from_file4()
        .expect("main -- read_username_from_file: hello3.txt");
    println!("s: {}", s);
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello3.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    // call the read_to_string method on the file handle in f
    // to read the contents of the file into s, pp. 178
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator, pp. 179
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello3.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// shorten the code further by chaining method calls
// immediately after the ?, pp. 180
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello3.txt")?
        .read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;
// using fs::read_to_string doesn’t give us the opportunity
// to explain all the error handling, pp. 181
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello3.txt")
}