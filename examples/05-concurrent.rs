use std::sync::{Arc, Mutex};
use std::thread::spawn;

fn main() {

    let data = Arc::new(Mutex::new(vec![]));
    let mut children = vec![];

    for i in 0..=3 {
        let vector = data.clone();
        children.push(spawn(move || {
            let mut vec = vector.lock().unwrap();
            vec.push(i);
            i
        }));
    }

    for child in children {
        println!("{}", child.join().unwrap());
    }

}