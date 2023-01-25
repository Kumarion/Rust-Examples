#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn match_example_func() {
    let age2 = 8;
    match age2 {
        1..=18 => println!("Important birthday"),
        21 | 50 => println!("Important birthday"),
        65..=i32::MAX => println!("Important birthday"),
        _ => println!("Not important birthday")
    };

    let my_age = 18;
    let voting_age = 18;

    match my_age.cmp(&voting_age) {
        Ordering::Less => println!("You can't vote yet."),
        Ordering::Greater => println!("You can vote."),
        Ordering::Equal => println!("You can vote."),
    };
}