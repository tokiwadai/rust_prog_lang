use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main() {
    let v = vec![1, 2, 3];


    /** The closure uses v, so it will capture v and make it part of the closure’s environment.
        Because thread::spawn runs this closure in a new thread,
        we should be able to access v inside that new thread. */
    /** Compile ERROR
        Rust infers how to capture v, and because println! only needs a reference to v,
        the closure tries to borrow v.
        However, there’s a problem: Rust can’t tell how long the spawned thread will run,
        so it doesn’t know if the reference to v will always be valid. */
//    let handle = thread::spawn(|| {
//        println!("Here's a vector: {:?}", v);
//    });


    /** If we were allowed to run this code, there’s a possibility
        the spawned thread would be immediately put in the background without running at all.
        The spawned thread has a reference to v inside, but the main thread immediately drops v.
        Then, when the spawned thread starts to execute, v is no longer valid, so a reference to it is also invalid.
        Oh no! pp. 397 */
//    drop(v); // oh no!


    /** By adding the move keyword before the closure,
        we force the closure to take ownership of the values it’s using
        rather than allowing Rust to infer that it should borrow the values, pp. 397 */
    let handle: JoinHandle<()> = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    /** If we added move to the closure, we would move v into the closure’s environment,
        and we could no longer call drop on it in the main thread, pp. 398 */
//    drop(v); // oh no!
    handle.join().unwrap();
}