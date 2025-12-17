use std::{sync::{Arc, Mutex, mpsc}, thread, time::Duration};

fn main() {
    basic_thread();
    shareing_data_in_thread();
    communicating_between_threads();
    sharing_mutable_data();
}

fn basic_thread() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Thread says: Hi Number {i}");
            thread::sleep(Duration::from_millis(200));
        }
    });

    // main thread
    for i in 1..5 {
        println!("Main thread say: hello number {i}");
        thread::sleep(Duration::from_millis(200));
    }

    handle.join().unwrap();
}

fn shareing_data_in_thread() {
    let mut data: Vec<i32> = Vec::new();

    let handle =thread::spawn(move || {
        for i in 1..5 {
            data.push(i);
            println!("Data is {:?}", data);
        }
    });
    
    handle.join().unwrap();
}

fn communicating_between_threads() {
    //Communicating Between Threads;
    let (sender_vec, receiver) = mpsc::channel::<&str>();

    let handle = thread::spawn(move || {
        let message:Vec<&str> = vec!["solana", "eth", "USDC","USDT"];

        for mes in message {
            sender_vec.send(mes).unwrap();
            println!("Thread send: {}", mes);
        }
    });

    for recive in receiver {
        println!("Thread receave in main: {}", recive);
    }

    handle.join().unwrap();
}

fn sharing_mutable_data() {
    // Sharing Mutable Data
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter_clone.lock().unwrap();
            *num +=  1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("The final answer: {}", *counter.lock().unwrap());

}