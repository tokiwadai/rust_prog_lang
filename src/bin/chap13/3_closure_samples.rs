/** When you create a closure, Rust infers which trait to use based on
    how the closure uses the values from the environment.
    All closures implement FnOnce because they can all be called at least once.
    Closures that don’t move the captured variables also implement FnMut,
    and closures that don’t need mutable access to the captured variables also implement Fn. */
fn main() {
    let x = 4;

    // Even though x is not one of the parameters of equal_to_x,
    // the closure is allowed to use the x variable
    // that’s defined in the same scope that is defined in, pp. 307
    let equal_to_x = |z| z == x;
    // The equal_to_x closure borrows x immutably (so equal_to_x has the Fn trait)
    // because the body of the closure only needs to read the value in x, pp. 309

    let y = 4;
    assert!(equal_to_x(y));



    let x = vec![1, 2, 3];
    // The x value is moved into the closure when the closure is defined,
    // because we added the move keyword.
    let equal_to_x = move |z| z == x;
    // If you want to force the closure to take ownership of the values it uses in the environment,
    // you can use the move keyword before the parameter list, pp. 309

    // The closure then has ownership of x, and below println isn’t allowed to use x anymore.
    // Removing println!, will fix this example.
//    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}