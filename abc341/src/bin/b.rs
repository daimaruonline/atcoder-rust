#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        mut A: [usize; N],
        ST: [(usize, usize); N-1],
    }
    for (i, st) in ST.iter().enumerate() {
        let count = A[i] / st.0;
        A[i] -= st.0 * count;
        A[i+1] += st.1 * count;
    }
    println!("{}", A[N-1]);
}
