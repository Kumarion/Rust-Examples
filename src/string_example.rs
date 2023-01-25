#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn string_example_func() {
    // String example
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    // Printing string split by whitespace
    for word in st1.split_whitespace() {
        println!("{}", word);
    };

    // Replacing string
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    // create string of random characters
    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1.iter() {
        print!("{}", char);
    };

    let st4: &str = "Random string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());

    st5.clear();

    let st7 = String::from("Just some");
    let st8 = String::from(" words");
    let st9 = st7 + &st8;

    for char in st9.bytes() {
        print!("{}", char);
    };
}