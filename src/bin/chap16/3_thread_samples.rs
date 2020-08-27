use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main() {
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    /** The main thread will wait for the spawned thread to finish and
        then run its for loop, so the output won’t be interleaved anymore, pp. 394 */
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}