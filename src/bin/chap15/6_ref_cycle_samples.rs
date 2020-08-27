use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    /** The second element in the Cons variant is now RefCell<Rc<List>>),
        meaning that instead of having the ability to modify the i32 value as we did in 5_rc_refcell_samples,
            enum List {
               Cons(Rc<RefCell<i32>>, Rc<List>),
                Nil,
            }

        we want to modify which List value a Cons variant is pointing to.
    */
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    // A tail method to make it convenient for us to access the second item if we have a Cons variant.
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    /** We create an Rc<List> instance holding a List value in the variable a with an initial list of 5, Nil. */
    let a: Rc<List> = Rc::new(Cons(5,RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    /** We then create an Rc<List> instance holding another List value in the variable b
        that contains the value 10 and points to the list in a, pp. 381 */
    let b: Rc<List> = Rc::new(Cons(10,RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    /** We modify a so it points to b instead of Nil, creating a cycle.
        We do that by using the tail method to get a reference to the RefCell<Rc<List>>.
        Then we use the borrow_mut method on the RefCell<Rc<List>> to change the value inside from an Rc<List>
        that holds a Nil value to the Rc<List> in b, pp. 381 */
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle, it will overflow the stack
//     println!("a next item = {:?}", a.tail());
}