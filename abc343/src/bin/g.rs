#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;

fn main() {
    input!{
        a: u32,
        b: u32,
    }
    let c: u32 = a * b;
    println!("{}", c);
}
