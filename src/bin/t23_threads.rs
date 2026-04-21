//! Topic 23 — Threads (TODO: expand with channels, Mutex, atomics, Send/Sync).

use std::thread;

fn main() {
    let h = thread::spawn(|| {
        println!("from child thread");
    });
    h.join().expect("join");
}
