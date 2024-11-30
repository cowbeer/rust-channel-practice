use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;

struct Actor {
    sender: Sender<Option<String>>,
    receiver: Mutex<Option<Receiver<Option<String>>>>,
    name: String,
    quiet: bool,
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl Actor {
    fn new(name: String, quiet: bool) -> Arc<Self> {
        let (sender, receiver) = channel();
        Actor {
            sender,
            receiver: Mutex::new(Some(receiver)),
            name,
            quiet,
            handle: <_>::default(),
        }
        .into()
    }

    fn start(self: &Arc<Self>) {
        let receiver = self.receiver.lock().unwrap().take().unwrap();
        let me = self.clone();
        // 创建线程，接收数据
        let handle = thread::spawn(move || {
            for message in receiver {
                if let Some(msg) = message {
                    if !me.quiet {
                        println!("{} received: {}", me.name, msg);
                    }
                } else { // received None
                    break;
                }
            }
            println!("{} stopped", me.name);
        });
        // 保存线程句柄
        self.handle.lock().unwrap().replace(handle);
    }

    fn send(&self, msg: String) {
        self.sender.send(Some(msg)).unwrap();
    }

    fn stop(&self) {
        if let Some(handle) = self.handle.lock().unwrap().take() {
            self.sender.send(None).unwrap();
            handle.join().unwrap();
        }
    }
}

fn main() {
    let actor1 = Actor::new("Actor1".to_string(), false);
    let actor2 = Actor::new("Actor2".to_string(), false);
    actor1.start();
    actor2.start();
    actor1.send("Hello from Main to Actor1".to_string());
    actor2.send("Hello from Main to Actor2".to_string());
    actor1.stop();
    actor2.stop();
}
