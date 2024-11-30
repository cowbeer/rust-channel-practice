use std::sync::mpsc;
use std::thread;

fn main() {
    let (sender, receiver) = mpsc::channel();
    for x in 1..10 {
        let sender_cloned = sender.clone();

        thread::spawn(move || {
            sender_cloned.send(x * x).unwrap();
        });
    }

    // close channel
    drop(sender);

    for x in receiver {
        println!("Received: {}", x);
    }
}
