#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

fn get_sum_generic<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

pub fn generics_func() {
    // Work with generics

    // Sum of integers
    println!("Sum: {}", get_sum_generic(5, 4));

    // Sum of floats
    println!("Sum: {}", get_sum_generic(5.5, 4.5));
}