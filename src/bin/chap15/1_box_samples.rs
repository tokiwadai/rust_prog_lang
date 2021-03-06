/** Compile error. recursive without indirection
    The error shows this type “has infnite size.”
    The reason is that we’ve de ned List with a variant that is recursive:
    it holds another value of itself directly.
    As a result, Rust can’t fgure out how much space it needs to store a List value, pp. 351 */
//enum List {
//    Cons(i32, List),
//    Nil,
//}

#[derive(Debug)]
enum ListBox {
    /** Because a Box is a pointer, Rust always knows how much space a Box needs:
        a pointer’s size doesn’t change based on the amount of data it’s pointing to, pp. 352 */
    Cons(i32, Box<ListBox>),
    Nil,
}


// Treating a Type Like a Reference by Implementing the Deref Trait, pp. 357
use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/** Without the Deref trait, the compiler can only dereference & references.
    The method gives the compiler the ability to take a value of any type that implements and
    call the deref method to get a & reference that it knows how to dereference, pp. 358 */
impl<T> Deref for MyBox<T> {
    type Target = T;

    /** We fill in the body of the deref method with &self.0
        so deref returns a reference to the value we want to access with the * operator, pp. 358 */
    fn deref(&self) -> &T {
        &self.0
    }
}

// Deref coercion
// A hello function that has the parameter name of type &str, pp. 359
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

//use crate::List::{Cons, Nil};
use crate::ListBox::{Cons, Nil};

fn main() {
    let b = Box::new(5);

    // Storing an i32 value on the heap using a box, pp. 348
    println!("b = {}", b);


    /** Compile error.
        The error shows this type “has infnite size.”
        The reason is that we’ve defined List with a variant that is recursive:
        it holds another value of itself directly.
        As a result, Rust can’t figure out how much space it needs to store a List value. */
//    let list = Cons(1, Cons(2, Cons(3, Nil)));


    let list: ListBox = Cons(1,
                             Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    println!("list: {:?}", list);


    /** Following the Pointer to the Value with the Dereference Operator, pp. 354
        The variable x holds an i32 value, 5.
        We set y equal to a reference to x. We can assert that x is equal to 5.
        However, if we want to make an assertion about the value in y,
        we have to use *y to follow the reference to the value it’s pointing to (hence dereference).
        Once we dereference y, we have access to the integer value y is pointing to
        that we can compare with 5, pp. 355 */
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);


    // Using Box<T> Like a Reference, pp. 355
    let x = 5;
    let y: Box<i32> = Box::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("Box y: {:?}", y);


    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    // behind the scenes Rust actually ran this code: *(y.deref()), pp. 358
    assert_eq!(5, *y);
    println!("MyBox y: {:?}", y);


    /**
     * Implicit Deref Coercions with Functions and Methods, pp. 359 */
    // call the hello function with a string slice as an argument, pp. 359
    hello("Rust1");

    // Calling hello with a reference to a MyBox<String> value,
    // which works because of deref coercion, pp. 359
    let m = MyBox::new(String::from("Rust2"));
    hello(&m);


    /** If Rust didn’t implement deref coercion,
        we would have to write the code like below instead,
        to call hello with a value of type &MyBox<String>, pp. 359
    */
    let m = MyBox::new(String::from("Rust3"));
    /** The (*m) dereferences the MyBox<String> into a String .
        Then the & and take a string slice of the String
        that is equal to the whole string to match the signature of hello, pp. 360 */
    hello(&(*m)[..]);
}