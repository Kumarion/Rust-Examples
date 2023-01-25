#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn ternary_operator_func() {
    let mut my_age = 47;
    let can_vote = if my_age >= 18 { true } else { false };
    println!("Can I vote? {}", can_vote);

    // Ternary operator
    let number_grade = 90;
    let letter_grade = if number_grade >= 90 { "A" } else if number_grade >= 80 { "B" } else if number_grade >= 70 { "C" } else if number_grade >= 60 { "D" } else { "F" };
    println!("Your letter grade is {}", letter_grade);
    println!("Your number grade is {}", number_grade);
}