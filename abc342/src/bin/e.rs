#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        M: usize,
        a: u32,
        b: u32,
    }
    let c: u32 = a * b;
    println!("{}", c);
}
