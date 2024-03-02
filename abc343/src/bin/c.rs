#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
// use std::collections::BinaryHeap;

fn main() {
    input!{
        N: usize,
    }
    let mut ans = 1;
    for x in 1..N {
        let K = x * x * x;
        if K > N {
            break;
        }
        let text = K.to_string();
        let reverse = text.chars().rev().collect::<String>();
        if text == reverse {
            ans = K;
        }
    }
    println!("{}", ans);
}
