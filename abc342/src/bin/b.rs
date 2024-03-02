#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        P: [usize; N],
        Q: usize,
        AB: [(usize, usize); Q],
    }
    let mut a: Vec<usize> = vec![0; N+1];
    for (i, p) in P.iter().enumerate() {
        a[*p] = i+1;
    }
    for ab in AB {
        if a[ab.0] > a[ab.1] {
            println!("{}", ab.1);
        } else {
            println!("{}", ab.0);
        }
    }
}
