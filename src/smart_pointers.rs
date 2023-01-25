#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::fmt;

// Pointer is basically just an address in memory
// Reference operator is & and dereference operator is *
// Smart pointers provide extra functionality over normal pointers
// Box smart pointer just stores data on the heap

// Stack : Stores values in a last in first out format
// Data on the stack must have a defined fixed size

// Binary tree example
//      50
//     /  \
//    30  70
//    /\   /\
//   10 60 80
//   /\   /\
// 5  15 65 90


pub fn smart_pointers_func() {
    // Quick box creation example
    let b_int1 = Box::new(10);
    println!("b_int1 = {}", b_int1);

    // Struct
    struct TreeNode<T> {
        pub left: Option<Box<TreeNode<T>>>,
        pub right: Option<Box<TreeNode<T>>>,
        pub key: T,
    }
    impl<T> TreeNode<T> {
        pub fn new(key: T) -> Self {
            TreeNode {left: None, right: None, key}
        }

        // Left
        pub fn left (mut self, node: TreeNode<T>) -> 
        Self {
            self.left = Some(Box::new(node));
            self
        }
        // Right
        pub fn right (mut self, node: TreeNode<T>) -> 
        Self {
            self.right = Some(Box::new(node));
            self
        }
    }

    // Create a tree
    let node1 = TreeNode::new(1)
        .left(TreeNode::new(2))
        .right(TreeNode::new(3));
    let node2 = TreeNode::new(4)
        .left(TreeNode::new(5))
        .right(TreeNode::new(6));
    let root = TreeNode::new(7)
        .left(node1)
        .right(node2);

}