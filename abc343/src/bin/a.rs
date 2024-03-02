#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;

fn main() {
    input!{
        A: usize,
        B: usize,
    }
    for i in 0..9 {
        if i != A+B {
            println!("{}", i);
            break;
        }
    }
}
