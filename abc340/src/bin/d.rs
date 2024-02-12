#![allow(non_snake_case)]
use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input!{
        N: usize,
    }
    let mut edge: Vec<Vec<(usize, usize)>> = vec![Vec::new(); N];
    for i in 0..(N-1) {
        input!{
            A: usize,
            B: usize,
            X: usize,
        }
        edge[i].push((i+1, A));
        edge[i].push((X-1, B));
    }

    let mut d = vec![isize::max_value(); N];
    let mut bh = BinaryHeap::new();
    let mut ok: Vec<isize> = vec![-1; N];

    d[0] = 0;
    bh.push((0, 0));
    ok[0] = 0;

    let mut count = 0;
    while !bh.is_empty() {
        let f = bh.pop().unwrap();
        let u = f.1;
        ok[u] = 1;
        if d[u] < f.0 * (-1) {
            continue;
        }

        for v in edge[u].iter() {
            if ok[v.0] != 1 {
                if d[v.0] > d[u] + v.1 as isize {
                    d[v.0] = d[u] + v.1 as isize;
                    bh.push((d[v.0]*(-1), v.0));
                    ok[v.0] = 0;
                }
            }
        }

        // println!("# loop:{}", count);
        // for i in 0..N {
        //     println!(" edge:{} d:{} ok:{}", i, d[i], ok[i]);
        // }
        // for b in &bh {
        //     println!(" BH: ({}, {})", b.0, b.1);
        // }
        // count+=1;
    }

    println!("{}", d[N-1]);
}
