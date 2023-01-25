#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn enumerations_func() {
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    };

    // Create functions for enums
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    };

    // Create a day
    let today:Day = Day::Monday;

    // Match day
    match today {
        Day::Monday => println!("Monday"),
        Day::Tuesday => println!("Tuesday"),
        Day::Wednesday => println!("Wednesday"),
        Day::Thursday => println!("Thursday"),
        Day::Friday => println!("Friday"),
        Day::Saturday => println!("Saturday"),
        Day::Sunday => println!("Sunday")
    };

    // Is today a weekend?
    println!("Is today a weekend? {}", today.is_weekend());
}