#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

pub fn arrays_func() {
    let arr_1 = [1, 2, 3, 4];

    // Get first element
    println!("1st element of arr_1 is {}", arr_1[0]);

    // Get length
    println!("Length of arr_1 is {}", arr_1.len());

    // Loop through array
    let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut loop_index = 0;

    // Loop through array with Loop keyword
    loop {
        if (arr_2[loop_index] % 2 == 0) {
            loop_index += 1;
            continue;
        };

        if (arr_2[loop_index] == 9) {
            break;
        };

        println!("[Array 2] Val : {}", arr_2[loop_index]);
        loop_index += 1;
    };

    // Loop through array with while loop
    let arr_3 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut while_index = 0;

    while (while_index < arr_3.len()) {
        println!("[Array 3] Val : {}", arr_3[while_index]);
        while_index += 1;
    };

    // Loop through array with for loop
    let arr_4 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    for element in arr_4.iter() {
        println!("[Array 4] Val : {}", element);
    };

}