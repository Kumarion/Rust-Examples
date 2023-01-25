#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// A closure is a function without a name
// They are more than likely going to be stored in a variable
// They can be passed as arguments to other functions

pub fn closures_func() {
    // let var_name = |param1, param2| -> return_type {BODY};
    let can_vote = |age: u8| -> bool {
        if (age >= 18) {
            true
        } else {
            false
        }
    };

    // Call the closure
    println!("Can vote: {}", can_vote(19));

    // Closures can access variables from the scope they are defined in
    let mut samp1 = 5;
    let print_var = || println!("samp1 = {}", samp1);
    print_var();
    // You can change values if you mark the variable as mutable
    samp1 = 10;
    let mut change_var = || samp1 += 1;
    change_var();
    println!("samp1 = {}", samp1);
    samp1 = 10;
    println!("samp1 = {}", samp1);

    // You can pass closures to functions aswell
    // The function will take ownership of the closure
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where T: Fn(i32, i32) -> i32, {
        func(a, b)
    };
    // Define the closures
    let sum = |a: i32, b: i32| -> i32 {a + b};
    let prod = |a: i32, b: i32| -> i32 {a * b};

    // Print the sum and product
    println!("5 + 5: Sum: {}", use_func(5, 5, sum));
    println!("5 * 5: Product: {}", use_func(5, 5, prod));
}