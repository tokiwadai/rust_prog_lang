fn main() {
    // 1. Mutable references, pp. 83
    let mut s = String::from("hello");
    let r1 = &mut s;

    /**
    * you can have only one mutable reference
    * to a particular piece of data in a particular scope
    *
    * Code below will fail
    */
//    let r2 = &mut s; // FAIL!!
    println!("r1: {}", r1);


    // 2. use curly brackets to create a new scope,
    // allowing for multiple mutable references, just not simultaneous ones, pp. 84
    let mut s = String::from("hello"); {
        let r1 = &mut s;
    } // r1 goes out of scope here,
      // so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r1: {}, r2: {}", r1, r2);


    // 2a. Can NOT combine mutable and immutable references, pp. 85
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // code below will faile
//    let r3 = &mut s; // BIG PROBLEM
    println!("r1: {}, r2: {}", r1, r2);


    // 2b. Note that a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used, pp. 86
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {}, r2: {}", r1, r2);

    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem println!("{}", r3);
    println!("r3: {}", r3);
}

