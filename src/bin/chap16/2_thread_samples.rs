use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread2!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread2!", i);
        thread::sleep(Duration::from_millis(1));
    }

    /** Calling join on the handle blocks the thread currently running
        until the thread represented by the handle terminates.
        Blocking a thread means that thread is prevented from performing work or exiting.

        The two threads continue alternating,
        but the main thread waits because of the call to handle.join() and
        does not end until the spawned thread is  nished., pp. 393 */
    handle.join().unwrap();
}