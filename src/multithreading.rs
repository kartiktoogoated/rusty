use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 0..5{
            println!("hi from spawned thread {}", i);
        }
    });

    for i in 0..5 {
        println!("hi from the main thread {}", i);
    }
    handle.join();
}

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Heres a vector: {:?}", v);
    });

    handle.join().unwrap();
}

use std::{sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..6 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..1000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);

    let mut ans = 0;
    for val in rx {
        ans = ans + val;
        println!("found value");
    }
    println!("Ans is {}", ans);
}