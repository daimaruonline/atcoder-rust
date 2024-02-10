#![allow(non_snake_case)]
use proconio::input;
use itertools::Itertools;

fn main() {
    input!{
        mut A: usize,
        B: usize,
        D: usize,
    }
    let mut ar: Vec<usize> = Vec::new();
    while A <= B {
        ar.push(A);
        A += D;
    }
    println!("{}", ar.iter().join(" "));
}
