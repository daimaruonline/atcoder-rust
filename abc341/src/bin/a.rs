#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
    }
    print!("1");
    for _i in 0..N {
        print!("01");
    }
    println!("");
}
