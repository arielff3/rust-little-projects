use std::{thread};

fn main() {
    let mut handles = Vec::new();

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread: {}", i);
        });
        handles.push(handle);
    };

    for h in handles {
        h.join().unwrap();
    }

    println!("All threads finished");
}
