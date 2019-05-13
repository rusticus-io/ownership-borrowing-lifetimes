use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {

    let data = Arc::new(Mutex::new(vec![]));
    let mut receiver = vec![];

    for i in 0..=3 {
        let vector = data.clone();
        receiver.push(spawn(move || {
            let mut vec = vector.lock().unwrap();
            vec.push(i);
            i
        }));
    }

    for r in receiver {
        println!("{}", r.join().unwrap());
    }

}