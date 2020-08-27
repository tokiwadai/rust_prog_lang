use std::cell::RefCell;
use std::rc::{Rc, Weak};

/**
    pp. 386
*/

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    /** leaf starts out without a parent, so we create a new, empty Weak<Node> reference instance, pp. 386 */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // get a reference to the parent of leaf by using the method, we get a None value.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let leaf2 = Rc::new(Node {
        value: 6,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf2 parent = {:?}", leaf2.parent.borrow().upgrade());


    let branch = Rc::new(Node { value: 50,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf), Rc::clone(&leaf2)]),
    });
    println!("branch: {:?}", branch);

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    /** The lack of infinite output indicates that this code didn’t create a reference cycle.
        We can also tell this by looking at the values we get from calling Rc::strong_count and Rc::weak_count */
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());


    println!("=====================");
    /** in an inner scope and examining strong and weak reference counts, pp. 388 */
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf), );
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("inner - branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch), );
        println!("inner - leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf), );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf), );
}