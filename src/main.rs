#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
mod constants;
mod asking_name;
mod data_types;
mod math;
mod random;
mod if_statements;
mod ternary_operator;
mod match_example;
mod arrays;
mod tuples_example;
mod string_example;
mod casting;
mod enumerations;
mod vectors;
mod functions;
mod generics;
mod ownership;
mod hash_maps_example;
mod structs;
mod modules_example;
mod iterators;
mod closures;
mod smart_pointers;
mod concurrency;

// Declare the restaurant module
mod restaurant;
use crate::restaurant::order_food;

mod error_handling;

fn main() {
    // // Constants
    // println!("Constants:");
    // constants::constants_func();

    // // Asking for name
    // println!("Asking for name:");
    // asking_name::asking_name_func();

    // // Data types
    // println!("Data types:");
    // data_types::data_types_func();

    // Math
    // println!("Math:");
    // math::math_func();

    // // Random
    // println!("Random:");
    // random::random_func();

    // // If statements
    // println!("If statements:");
    // if_statements::if_statements_func();

    // // Ternary operator
    // println!("Ternary operator:");
    // ternary_operator::ternary_operator_func();

    // // Match example
    // println!("Match example:");
    // match_example::match_example_func();

    // // Arrays
    // println!("Arrays:");
    // arrays::arrays_func();

    // // Tuples example
    // println!("Tuples example:");
    // tuples_example::tuples_example_func();

    // // String example
    // println!("String example:");
    // string_example::string_example_func();

    // // Casting
    // println!("Casting:");
    // casting::casting_func();

    // // Enumerations
    // println!("Enumerations:");
    // enumerations::enumerations_func();

    // // Vectors
    // println!("Vectors:");
    // vectors::vectors_func();

    // // Functions
    // println!("Functions:");
    // functions::functions_func();

    // // Generics
    // println!("Generics:");
    // generics::generics_func();

    // // Ownership
    // println!("Ownership:");
    // ownership::ownership_func();

    // // Hash maps
    // println!("Hash maps:");
    // hash_maps_example::hash_maps_func();

    // // Structs
    // println!("Structs:");
    // structs::structs_func();

    // // Modules
    // println!("Modules:");
    // modules_example::modules_example_func();
    // // Run the order_food function from the restaurant module
    // order_food();

    // // Error Handling
    // println!("Error Handling:");
    // error_handling::error_handling_func();

    // // Iterators
    // println!("Iterators:");
    // iterators::iterators_func();

    // // Closures
    // println!("Closures:");
    // closures::closures_func();

    // // Smart pointers
    // println!("Smart pointers:");
    // smart_pointers::smart_pointers_func();

    // // Concurrency
    // println!("Concurrency:");
    // concurrency::concurrency_func();
}