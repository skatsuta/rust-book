use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let handles = (0..10).map(|_| {
        let counter = counter.clone();
        thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
