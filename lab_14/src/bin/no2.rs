use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
        }
    });

    for received in rx.iter() {
        println!("Received: {}", received);
    }
}
