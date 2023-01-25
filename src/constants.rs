#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn constants_func() {
    const ONE_MILLION: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "18";
    let mut age: u32 = age.trim().parse()
        .expect("Age not a number");

    // Add one for 19
    age = age + 1;

    println!("I am {} and I want ${}", age, ONE_MILLION);
}