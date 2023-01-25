#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Stack : Stores values in a last in first out format
// Data on teh stack must have a defined fixed size

// Heap : When putting data on the help you request a certain amount of space. The OS finds space available and returns an address for that space called a pointer;
// The pointer is stored on the stack and the data is stored on the heap

// RULES
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

fn print_str(x: String) {
    println!("A string {}", x);
}


fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_str(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}

pub fn ownership_func() {
    // Using the stack
    let str1 = String::from("World");
    let str2 = str1;
    
    // println!("str1: {}", str1); // This will not work because str1 is no longer valid
    println!("str2: {}", str2); // This will work because str2 is valid

    // Using cloning
    let str3 = String::from("Hello");
    let str4 = str3.clone();
    println!("str3: {}", str3); // This will work because str3 is valid
    println!("str4: {}", str4); // This will work because str4 is valid

    // Wont apply to integers because they are stored on the stack
    // Will apply to strings because they are stored on the heap

    // Printing strings using our functions
    let str5 = String::from("Hello");
    print_str(str5);

    // Returning strings using our functions
    let str6 = String::from("Hello");
    let str7 = print_return_str(str6);

    // Pushing to a string
    let mut name_str = String::from("Jack");
    change_str(&mut name_str);
}