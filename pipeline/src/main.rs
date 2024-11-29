use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // use a scope to avoid having to join all the threads
    thread::scope(|scope| {
        let (sender, receiver) = channel();
        let (sender2, receiver2) = channel();

        // stage1
        scope.spawn(move || {
            let data = vec![1, 2, 3, 4, 5];
            for x in data {
                sender.send(x).unwrap();
                sleep(Duration::from_secs(1));
            }
        });

        // stage2
        scope.spawn(move || {
            for x in receiver {
                sender2.send(x * x).unwrap();
                sleep(Duration::from_secs(1));
            }
        });

        // final
        scope.spawn(move || {
            for x in receiver2 {
                println!("Received: {}", x);
            }
        });
    });
}
