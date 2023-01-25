#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Iterators are used for looping through things like:
// - Vectors
// - Arrays
// - Tuples
// - Strings
// - Hash maps
// - Files
// - etc.

// Iterators are lazy, meaning they don't do anything until you call a method on them

pub fn iterators_func() {
    let mut arr_it = [1, 2, 3, 4];

    // Iterate over the array
    for val in arr_it.iter() {
        println!("Value: {}", val);
    };

    // Can use the collection but it will take ownership
    arr_it.into_iter().for_each(|x| println!("Value: {}", x));

    // Iterate over the array and mutate the values
    let mut iter1 = arr_it.iter();
    println!("1st : {:?}", iter1.next());
}