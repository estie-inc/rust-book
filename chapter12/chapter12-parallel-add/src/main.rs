use std::{
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let counter = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..2_5000_0000 {
                let mut writer = counter.lock().unwrap();
                *writer += 1;
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {}", counter.as_ref().lock().unwrap());
}
