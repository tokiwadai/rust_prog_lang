use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        println!("sending: \"{}\"", val);
        /** The send function takes ownership of its parameter,
            and when the value is moved, the receiver takes ownership of it.
            This stops us from accidentally using the value again after sending it;
            the ownership system checks that everything is okay, pp. 402 */
        tx.send(val).unwrap();
//        Compile Error!!
//        Try to use a val value in the spawned thread after we’ve sent it down the channel, pp. 402
//        println!("val is {}", val);
//                              ^^^ value borrowed here after move
    });

    /** recv , short for receive, which will block the main thread’s execution and
        wait until a value is sent down the channel.
        Once a value is sent, recv will return it in a Result<T, E>.
        When the sending end of the channel closes,
        recv will return an error to signal that no more values will be coming, pp. 401 */
    let received = rx.recv().unwrap();
    println!("receiving: \"{}\"", received);


    println!("/** Sending multiple messages and pausing between each, pp. 403 */");
    let (tx, rx) = mpsc::channel();
    /** The spawned thread will now send multiple messages
        and pause for a second between each message, pp. 403 */
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        /** We iterate over them, sending each individually,
            and pause between each by calling the thread::sleep function
            with a Duration value of 1 second */
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    /** Treating rx as an iterator
        For each value received, we’re printing it.
        When the channel is closed, iteration will end. */
    for received in rx {
        println!("Got: {}", received);
    }


    println!("/** Creating Multiple Producers by Cloning the Transmitter, pp. 404 */");
    let (tx, rx) = mpsc::channel();
    /** Use mpsc to create multiple threads that all send values to the same receiver.
        We can do so by cloning the transmitting half of the channel */
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}