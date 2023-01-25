#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// Custom data type that can store multiple different types of data
// Kind of like a class

pub fn structs_func() {
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    };

    // Create a customer
    let mut bob = Customer {
        name: String::from("Bob"),
        address: String::from("123 Main St"),
        balance: 100.00,
    };

    // Change a value
    bob.address = String::from("456 Main St");

    // Print a value
    println!("{}'s address is {}", bob.name, bob.address);


    // Traits
    const PI: f32 = 3.14159265359;
    // A trait is a collection of methods that can be used by multiple data types
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area (&self) -> f32;
    };

    // Rectangle
    struct Rectange {length : f32, width: f32};
    struct Circle {length : f32, width: f32};

    // Implement the Shape trait for the Rectangle struct
    impl Shape for Rectange {
        fn new (length : f32, width: f32) -> Rectange {
            return Rectange {length, width}
        }
        fn area (&self) -> f32 {
            return self.length * self.width;
        }
    };

    // Implement the Shape trait for the Circle struct
    impl Shape for Circle {
        fn new (length : f32, width: f32) -> Circle {
            return Circle {length, width}
        }
        fn area (&self) -> f32 {
            return (self.length / 2.0).powf(2.0) * PI
        }
    };

    // Create a rectangle
    let rect = Rectange::new(10.0, 10.0);
    println!("The area of the rectangle is {}", rect.area());

    // Create a circle
    let circle = Circle::new(10.0, 5.0);
    println!("The area of the circle is {}", circle.area());
}