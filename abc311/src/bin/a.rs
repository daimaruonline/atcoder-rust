#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        N: usize,
        S: Chars,
    }
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for (i, s) in S.iter().enumerate() {
        if s == &'A' {
            a += 1;
        } else if s == &'B' {
            b +=1;
        } else if s == &'C' {
            c += 1;
        }
        if a > 0 && b > 0 && c > 0 {
            println!("{}", i+1);
            break;
        }
    }
}
