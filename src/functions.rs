#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn get_sum(x: i32, y: i32) -> i32 {
    x + y
}

fn say_hello () {
    println!("Hello!");
}

fn return_multiple(x: i32, y: i32) -> (i32, i32) {
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &x in list.iter() {
        sum += &x;
    }
    sum
}

pub fn functions_func() {
    // Hello
    say_hello();
    
    // Sum
    let sum = get_sum(5, 5);
    println!("Sum: {}", sum);

    // Return multiple
    let (a, b) = return_multiple(1, 2);
    println!("a: {}, b: {}", a, b);

    // Number list
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("Sum of list: {}", sum_list(&num_list));
}