use std::process;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    println!("\n=> Running join example");
    thread_join_example();
    println!("\n=> Running channels example");
    channel_example();
    println!("\n=> Running mutex example");
    mutex_example();
    println!("\n=> Running loop example");
    thread_loop_example();
}

fn thread_loop_example() {
    let mut loop_iteration_num = 1;

    loop {
        if loop_iteration_num > 5 {
            println!("Enough iterations!");
            process::exit(0);
        }

        println!("start loop iteration {}", loop_iteration_num);

        if loop_iteration_num % 2 == 0 {
            spawn_thread();
        }

        for i in 1..5 {
            println!("MAIN THREAD number {}", i);
            thread::sleep(Duration::from_millis(1000));
        }

        loop_iteration_num = loop_iteration_num + 1
    }
}

fn thread_join_example() {
    let handle = spawn_thread();

    handle.join().unwrap();

    for i in 1..5 {
        println!("MAIN THREAD number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn spawn_thread() -> JoinHandle<()> {
    thread::spawn(|| {
        for i in 1..10 {
            println!("SPAWNED THREAD number {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    })
}

fn channel_example() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn mutex_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
