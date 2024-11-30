use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

type SubscriberId = usize;
struct PubSub<T> {
    subscribers: Arc<Mutex<BTreeMap<SubscriberId, SyncSender<T>>>>,
    next_id: AtomicUsize,
}

#[allow(dead_code)]
impl<T: Send + 'static> PubSub<T> {
    fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: <_>::default(),
        }
    }

    fn subscribe(&self) -> (SubscriberId, Receiver<T>) {
        let (sender, receiver) = sync_channel(512);
        let mut subscribers = self.subscribers.lock().unwrap();
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        subscribers.insert(id, sender);
        (id, receiver)
    }

    fn unsubscribe(&self, id: SubscriberId) {
        let mut subscribers = self.subscribers.lock().unwrap();
        subscribers.remove(&id);
    }

    // an example of wrong implementation: deadlocks the whole hub if any channel is full.
    fn publish_deadlocking(&self, message: T)
    where
        T: Clone,
    {
        let subscribers = self.subscribers.lock().unwrap();
        for sender in subscribers.values() {
            sender.send(message.clone()).unwrap();
        }
    }

    // an example of correct implementation: subscribers are cloned before sending and the lock is released instantly.
    // may lead to tiny overhead with cloning and data-races when a subscriber may receive a message after it has been unsubscribed,
    // however the deadlock-free benefits fully compensate for all of that
    fn publish(&self, message: T)
    where
        T: Clone,
    {
        let subscribers = self.subscribers.lock().unwrap().clone();
        for sender in subscribers.values() {
            let _ = sender.send(message.clone());
        }
    }
}
impl<T> Drop for PubSub<T> {
    fn drop(&mut self) {
        drop(self.subscribers.lock().unwrap())
    }
}

fn main() {
    let hub: PubSub<i32> = PubSub::new();

    let (sub_id, receiver) = hub.subscribe();
    let (sub_id2, receiver2) = hub.subscribe();

    let handle = thread::spawn(move || {
        for received in receiver {
            println!("Subscriber1 Received: {}", received);
        }
    });

    let handle2 = thread::spawn(move || {
        for received in receiver2 {
            println!("Subscriber2 Received: {}", received);
        }
    });

    // drop(hub);
    // println!("Hub dropped");

    for i in 0..20 {
        if i == 7 {
            hub.unsubscribe(sub_id);
        }
        if i == 14 {
            hub.unsubscribe(sub_id2);
        }
        hub.publish(i);
        sleep(Duration::from_secs(1));
    }

    // hub.unsubscribe(sub_id2);

    handle.join().unwrap();
    handle2.join().unwrap();
}
