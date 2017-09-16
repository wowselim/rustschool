use std::{thread, time};
use std::sync::mpsc::channel;

fn main() {
    let (w, r) = channel();

    thread::spawn(move || {
        println!("sending name from thread");
        let _ = w.send("Selim");
    });

    // thread::sleep(time::Duration::from_millis(1000));
    // alternative: thread::slep_ms(u32) :)

    println!("Waiting for name in main thread");
    println!("Hello {}", r.recv().unwrap());
}
