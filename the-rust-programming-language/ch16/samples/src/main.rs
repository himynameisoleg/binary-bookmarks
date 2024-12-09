// use std::thread;
// use std::time::Duration;
//
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi the number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     handle.join().unwrap();
//
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }

// // Using move Closures with Threads
// use std::thread;
//
// fn main() {
//     let v = vec![1, 2, 3];
//
//     let handle = thread::spawn(move || {
//         println!("Here is a vector: {:?}", v);
//     });
//
//     handle.join().unwrap();
// }

// use std::{sync::mpsc, thread, time::Duration};
//
// // safely passing messages between Threads
// fn main() {
//     let (tx, rx) = mpsc::channel();
//
//     let tx1 = mpsc::Sender::clone(&tx);
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];
//
//         for val in vals {
//             tx1.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("more"),
//             String::from("messages"),
//             String::from("for"),
//             String::from("you"),
//         ];
//
//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });
//
//     for received in rx {
//         println!("Got: {}", received);
//     }
// }

use std::sync::Mutex;

// Using Mutexes
fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}
