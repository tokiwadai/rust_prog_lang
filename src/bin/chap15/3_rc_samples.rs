#[derive(Debug)]
enum List_Box {
    Cons(i32, Box<List_Box>),
    Nil,
}

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

fn main() {
    /** Box<T> allows immutable or mutable borrows checked at compile time, pp. 370 */
    let a = List_Box::Cons(5,
                           Box::new(List_Box::Cons(10,
                                                   Box::new(List_Box::Nil))));
    let b = List_Box::Cons(3, Box::new(a));
    /** The Cons variants own the data they hold, so when we create the b list,
        a is moved into b and b owns a. Then, when we try to use a again when creating c,
        we’re not allowed to because a has been moved, pp. 366
    */
//    let c = List::Cons(4, Box::new(a));
    println!("b1: {:?}", b);


    /** pp 370
        - Rc<T> ENABLES MULTIPLE OWNERS of the same data,
          while Box<T> and RefCell<T> have single owners
        - Rc<T> allows only IMMUTABLE borrows checked at compile time
        */
    let a = Rc::new(List::Cons(5,
                               Rc::new(List::Cons(10,
                                                  Rc::new(List::Nil)))));
    /** The implementation of Rc::clone doesn’t make a deep copy of all the data
        like most types’ implementations of clone do.
        The call to Rc::clone only increments the reference count, which doesn’t take much time.
        Deep copies of data can take a lot of time, pp. 367 */
    let b = List::Cons(3, Rc::clone(&a));
    // c will own the same data a as b does
    let c = List::Cons(4, Rc::clone(&a));
    println!("b2: {:?}", b);
    println!("c2: {:?}", c);


    /** Printing the reference count, pp. 368 */
    let a = Rc::new(List::Cons(5,
                               Rc::new(List::Cons(10,
                                                  Rc::new(List::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = List::Cons(3, Rc::clone(&a));

    println!("count after creating b = {}", Rc::strong_count(&a)); {
        let c = List::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}