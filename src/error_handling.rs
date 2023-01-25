#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Bunch of ways to handle errors
// https://doc.rust-lang.org/book/ch09-00-error-handling.html
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
// https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
// https://doc.rust-lang.org/book/ch09-04-unrecoverable-errors-with-panic.html

// Result has 2 varients Ok and Err
// enum Result<T, E> {
    // Ok(T),
    // Err(E),
// }
// Where T represents the data typeof the value returns and E
// the type of the error

pub fn error_handling_func() {
   let path = "lines.txt";
   let output = File::create(path);
   let mut output = match output {
      Ok(file) => file,
      Err(error) => {
        panic!("Problem creating the file: {:?}", error);
      }
   };

   // Shorter way to write the above match
   write!(output, "Just some\nRandom words")
        .expect("Couldn't write to file");

   let input = File::open(path).unwrap();
   let buffered = BufReader::new(input);

   for line in buffered.lines() 
   {
        println!("{}", line.unwrap());
   };

   let output2 = File::create("rand.txt");
   let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _other_error => panic!("Problem opening the file: {:?}", _other_error),
        },
    };
}