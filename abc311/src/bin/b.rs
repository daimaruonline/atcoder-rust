#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        N: usize,
        D: usize,
        S: [Chars; N],
    }
    let mut res = 0;
    let mut c = 0;
    for d in 0..D {
        let mut found = true;
        for n in 0..N {
            if S[n][d] != 'o' {
                found = false;
                break;
            }
        }
        if found == true {
            // println!("Days:{} success {}->{}", d, c, c+1);
            c += 1;
            if c > res {
                res = c;
            }
        } else {
            // println!("Days:{} failed", d);
            c = 0;
        }
    }
    println!("{}", res);
}
