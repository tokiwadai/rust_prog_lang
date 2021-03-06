
// expected named lifetime parameter, pp. 217
/**
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

// Lifetime Annotations in Function Signatures, pp. 218
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// The way in which you need to specify lifetime parameters
// depends on what your function is doing.
// For example, if we changed the implementation of the longest function
// to always return the  rst parameter rather than the longest string slice,
// we wouldn’t need to specify a lifetime on the y parameter.
fn longest_x<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

use std::fmt::Display;
// an extra parameter named ann of the generic type T,
// which can be filled in by any type that implements the Display trait
// as speci ed by the where clause, pp. 227
// This extra parameter will be printed before the function compares the lengths of the string slices,
// which is why the Display trait bound is necessary, pp. 228
fn longest_with_an_announcement<'a, T>( x: &'a str,
                                        y: &'a str,
                                        ann: T, ) -> &'a str
        where T: Display, {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let string1 = String::from("long string is long");
    /** This won't compile, pp. 220
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest_with_an_announcement(string1.as_str(), string2, "Hello!!");
    println!("The longest string is {}", result);
}