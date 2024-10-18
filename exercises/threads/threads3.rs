// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Arc<Vec<u32>>,
    second_half: Arc<Vec<u32>>,

}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: Arc::new(vec![1, 2, 3, 4, 5]),
            second_half: Arc::new(vec![6, 7, 8, 9, 10]),
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let first_half_clone = Arc::clone(&q.first_half);
    let second_half_clone = Arc::clone(&q.second_half);

    let tx_first_half = tx.clone();
    thread::spawn(move || {
        for val in &*first_half_clone{
            println!("sending {:?}", val);
            tx_first_half.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let tx_second_half = tx.clone();
    thread::spawn(move || {
        for val in &*second_half_clone {
            println!("sending {:?}", val);
            tx_second_half.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
