use std::sync::Mutex;
use std::thread;
use std::sync::MutexGuard;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("/** a mutex in a single-threaded context, pp. 407 */");
    let m: Mutex<i32> = Mutex::new(5); {
        /** To access the data inside the mutex, we use the method to acquire the lock.
            This call will block the current thread so it can’t do any work
            until it’s our turn to have the lock.

            The call to lock would fail if another thread holding the lock panicked.
            In that case, no one would ever be able to get the lock,
            so we’ve chosen to unwrap and have this thread panic in that situation, pp. 407 */
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
    }
    /** Mutex<T> is smart pointer.
        The call to lock returns a MutexGuard wrapped in a LockResult
        that we handled with the call to unwrap.

        The MutexGuard implements Deref to point at our inner data;
        the smart pointer also has a Drop implementation that releases the lock automatically
        when a MutexGuard goes out of scope, which happens at the end of the inner scope.

        As a result, we don’t risk forgetting to release the lock and
        blocking the mutex from being used by other threads
        because the lock release happens automatically, pp. 408 */
    println!("m = {:?}", m);


    /** Sharing a Mutex<T> Between Multiple Threads, pp. 408 */
    /** Compile Error
        The error message states that the counter value was moved
        in the previous iteration of the loop.
        So Rust is telling us that we can’t move the ownership of lock counter
        into multiple threads

        let handle = thread::spawn(move || {
                                   ^^^^^^^ value moved into closure here, in previous iteration of loop
        */
//    let counter: Mutex<i32> = Mutex::new(0);

    /** Compile Error
        Unfortunately, Rc<T> is not safe to share across threads.
        When Rc<T> manages the reference count,
        it adds to the count for each call to clone and subtracts from the count
        when each clone is dropped.

        let handle = thread::spawn(move || {
                     ^^^^^^^^^^^^^
                     std::rc::Rc<std::sync::Mutex<i32>> cannot be sent between threads safely
        */
    /**
        let counter = Rc::clone(&counter);

        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(move || {
                let mut num: MutexGuard<i32> = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Result: {}", *counter.lock().unwrap());
    */

    println!("/** Atomic Reference Counting with Arc<T>, pp. 411 */");
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num: MutexGuard<i32> = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());

}