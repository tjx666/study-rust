#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn thread_capture() {
    let v = vec![1, 2, 3];

    let task = move || {
        println!("Here's a vector: {:?}", v);
    };

    let handle = thread::spawn(task);

    // drop(v); // oh no!

    handle.join().unwrap();
}

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let mut handles = vec![];

    // handle.join().unwrap();
    let counter = Arc::new(Mutex::new(0));
    for i in 0..10 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1
        }))
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}
