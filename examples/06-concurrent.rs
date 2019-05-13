use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

static NTHREADS: i32 = 3;

fn main() {
    let mut rng = rand::thread_rng();

    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    let mut children = Vec::new();

    for id in 0..NTHREADS {
        let thread_tx = tx.clone();

        let millis = rng.gen_range(400, 1000);

        let child = thread::spawn(move || {
            sleep(Duration::from_millis(millis));
            thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(NTHREADS as usize);
    for _ in 0..NTHREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("{:?}", ids);
}
