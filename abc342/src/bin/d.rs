#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        A: [i64; N],
    }
    let mut ans = 0;
    for i in 0..N {
        if A[i] == 0 {
            ans += N-i-1;
            // println!("{}", i);
            continue;
        }
        for j in (i+1)..N {
            let a: i64 = (A[i] * A[j]) as i64;
            let b: i64 = f64::sqrt(a as f64) as i64;
            if a == (b*b) {
                // println!("{} {}", i, j);
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
