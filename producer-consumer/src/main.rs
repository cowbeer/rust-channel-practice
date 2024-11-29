use std::sync::mpsc;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (sender, receiver) = mpsc::channel();

    let producer = thread::spawn(move || {
        let data = vec![1, 2, 3, 4, 5];
        for x in data {
            sender.send(x).unwrap();
            println!("Sent {}", x);
            sleep(Duration::from_secs(1));
        }
    });

    let consumer = thread::spawn(move || {
        for x in receiver {
            println!("Received: {}", x);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}
