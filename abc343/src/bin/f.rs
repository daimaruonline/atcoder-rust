#![allow(non_snake_case)]
use proconio::input;
// use proconio::marker::Bytes;
// use proconio::marker::Chars;
// use itertools::Itertools;
// use std::collections::HashMap;
use std::collections::BinaryHeap;

fn main() {
    input!{
        N: usize,
        Q: usize,
        mut A: [usize; N],
    }
    for i in 0..Q {
        input!{
            query: [usize; 3],
        }
        if query[0] == 1 {
            let p = query[1]-1;
            let x = query[2];
            A[p] = x;
        } else {
            let l = query[1]-1;
            let r = query[2]-1;

            let mut ans = 0;
            // println!("{:?}", &A[l..=r]);
            if A[l..=r].len() > 1 {
                // let bh: BinaryHeap<usize> = A[l..r].into_iter().collect();
                let mut bh: BinaryHeap<usize> = BinaryHeap::new();
                for a in &mut A[l..=r] {
                    bh.push(*a);
                }
                // println!("{:?}", bh);
                let first = bh.pop().unwrap();
                let mut second = first;
                while let Some(e) = bh.pop() {
                    if e == first {
                        continue;
                    }
                    if second == first {
                        second = e;
                    }
                    if second == e {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
            println!("{}", ans);
        }
    }
}
