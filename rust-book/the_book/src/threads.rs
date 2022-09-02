use std::sync::mpsc;
use std::{thread, time::Duration};
pub fn run_code_simultaneously() {
    // creating a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates
    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running. T

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

pub fn transfer_data_between_threads() {
    let (tx, rx) = mpsc::channel();
    // creating multiple producers by cloning the transmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let val = String::from("hi");
        // tx.send(val).unwrap();   The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it

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

    // let received = rx.recv().unwrap();
    // println!("got: {}", received);
    for received in rx {
        println!("Got: {}", received);
    }
}
