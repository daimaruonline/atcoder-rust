#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;

fn main() {
    input!{
        N: usize,
        A: [[usize; N]; N],
    }
    for i in 0..N {
        let mut res = Vec::new();
        for j in 0..N {
            if A[i][j] == 1 {
                res.push(j+1);
            }
        }
        println!("{}", res.iter().join(" "));
    }
}
