#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn hash_maps_func() {
    // Create hash map
    let mut heroes = HashMap::new();

    // Add values to hash map
    heroes.insert("Superman", "Clark Kent");
    heroes.insert("Batman", "Bruce Wayne");
    heroes.insert("Spiderman", "Peter Parker");
    heroes.insert("The Flash", "Barry Allen");

    // Print hash map
    for (hero, real_name) in &heroes {
        println!("{} is actually {}!", hero, real_name);
    }

    // Check if key exists
    // If it does, print a message
    // If it doesn't, print a message
    if heroes.contains_key(&"Batman") {
        let the_batman = heroes.get(&"Batman");

        match the_batman {
            Some(x) => println!("Batman is a hero!"),
            None => println!("Batman is not a hero!"),
        }
    };

    // Length of hash map
    println!("There are {} heroes in the hash map", heroes.len());
}