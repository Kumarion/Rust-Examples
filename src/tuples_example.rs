#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn tuples_example_func() {
    let my_tuple: (u8, String, f64) = (18, "Hello".to_string(), 50_000.00);

    // Get first element
    println!("Name: {}", my_tuple.1);

    // Destructuring
    let (v1, v2, v3) = my_tuple;
    println!("Age: {}", v1);
    println!("Float Number: {}", v3);
}