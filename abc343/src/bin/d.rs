#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashSet;
use std::collections::HashMap;
// use std::collections::BinaryHeap;

fn main() {
    input!{
        N: usize,
        T: usize,
    }
    let mut point: Vec<usize> = vec![0; N];
    let mut ans: HashMap<usize, usize> = HashMap::new();
    ans.insert(0, N);
    for i in 0..T {
        input!{
            A: usize,
            B: usize,
        }
        let before = point[A-1];
        let after = before + B;
        point[A-1] += B;
        // println!("{:?}", point);

        let before_count = ans.entry(before).or_insert(0);
        *before_count -= 1;
        if *before_count == 0 {
            ans.remove(&before);
        }
        let after_count = ans.entry(after).or_insert(0);
        *after_count += 1;
        println!("{}", ans.len());
    }
}
