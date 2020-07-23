fn main() {
    // Create an empty string, pp. 152
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // use the to_string method,
    // which is available on any type that implements the Display trait,
    // as string literals do, pp. 152
    let s = "initial contents".to_string();

    let s = String::from("initial contents");


    // strings are UTF-8 encoded,
    // so we can include any properly encoded data in them, pp. 153
    let hello = String::from("السلام علیكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    let mut s1 = String::from("foo");
    let s2 = " bar";
    // push_str method takes a string slice because we don’t necessarily
    // want to take ownership of the parameter, pp. 154
    s1.push_str(s2);
    // If the push_str method took ownership of s2,
    // we wouldn’t be able to print its value below
    println!("s2 is {}", s2);

    println!("s1 is {}", s1);


    let mut s = String::from("lo");
    // The push method takes a single character as a parameter
    // and adds it to the String, pp. 154
    s.push('l');
    println!("s is {}", s);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
//    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
//    println!("s1 is {}", s1); // This will fail since s1 is moved here
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
    println!("s3 is {}", s3);
    println!("s is {}", s);

    for c in "東京都にっぽん".chars() {
        println!("{}", c);
    }
    for c in "東京都にっぽん".bytes() {
        println!("{}", c);
    }
}

