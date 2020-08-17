/** Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    Rc<T> lets you have multiple owners of some data,
    but it only gives immutable access to that data.
    If you have an Rc<T> that holds a RefCell<T>,
    you can get a value that can have multiple owners and that you can mutate, pp. 377 */

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));
    let a: Rc<List> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b: List = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c: List = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    // We add 10 to the value in value.
    // We do this by calling borrow_mut on value, which uses the automatic dereferencing feature.
    // The borrow_mut method returns a RefMut<T> smart pointer,
    // and we use the dereference operator on it and change the inner value, pp. 378
    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}