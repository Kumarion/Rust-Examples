#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::thread;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Mutex, Arc};

// Parallelism is when you have multiple threads running at the same time
// Different blocks of code that execute are referred to as threads
// Threads handle the scheduling aswell as the execution of the code

// Common problems with parallel programming involve:
// 1. Threads are accessing data in the wrong order
// 2. Threads are blocked from executing because of confusion
// over requirements to proceed with execution

// No guarantees are made about the order in which threads will execute

pub fn concurrency_func() {
    // Create spawnwd thread
    let thread1 = thread::spawn(|| {
        for i in 1..10 {
            println!("Spawned threat: {}", i);
            thread::sleep(Duration::from_millis(1));
        };
    });

    // Main thread
    for i in 1..15 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    };

    // Guarantee that the spawned thread will finish before the main thread
    // This is done by calling the join method on the handle to the thread
    // This will cause the main thread to wait for the spawned thread to finish
    // before it continues
    thread1.join().unwrap();

    // Real world example
    pub struct Bank {
        balance: f32
    };

    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt:f32){
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00{
            println!("Current Balance : {} Withdrawal a smaller amount",
            bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}",
            amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> =
      Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {

        // Clone duplicates an the bank object
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    // Join all the threads
    for handle in handles {
        handle.join().unwrap();
    };

    // Print the final balance
    println!("Total {}", bank.lock().unwrap().balance);
}