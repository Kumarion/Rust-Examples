#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn random_func() {
    let range = 1..25;

    let random_num = rand::thread_rng()
        .gen_range(range);

    // Create loop for them to continue guessing, if they fail
    loop {
        println!("Guess the number!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Too small! Keep going."),
            Ordering::Greater => println!("Too big! Keep going."),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    };

    println!("The random number was: {}", random_num);
}