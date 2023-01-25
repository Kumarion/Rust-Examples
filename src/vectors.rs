#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn vectors_func() {
    // Create a static vector
    let vec1: Vec<i32> = Vec::new();

    // Create a dynamic vector
    let mut vec2 = vec![1, 2, 3, 4];

    // Add value to end of vector
    vec2.push(5);
    
    // Get a value from a specific index
    println!("Value at index 1: {}", vec2[0]);

    // Check if values exist
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd: {}", second),
        None => println!("No second value"),
    };

    // Cycle through and multiply values * 2
    for x in &mut vec2 {
        *x *= 2;
    };
    for x in &vec2 {
        println!("Value: {}", x);
    };

    // Get length of vector
    println!("Length: {}", vec2.len());
    
    // Pop
    println!("Popped: {:?}", vec2.pop());
}