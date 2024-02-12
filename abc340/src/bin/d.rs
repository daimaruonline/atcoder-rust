#![allow(non_snake_case)]
use proconio::input;

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

    let mut d = vec![usize::max_value(); N];
    d[0] = 0;
    let mut p: Vec<isize> = vec![-1; N];
    let mut ok: Vec<isize> = vec![-1; N];
    let mut min_node = 0;
    loop {
        let mut min_cost = usize::max_value();
        for i in 0..N {
            if ok[i] != 1 && d[i] < min_cost {
                min_cost = d[i];
                min_node = i;
            }
        }

        if min_cost == usize::max_value() {
            break;
        }
        ok[min_node] = 1;

        for &e in &edge[min_node] {
            let v = e.0;
            if ok[v] != 1 {
                if d[min_node] + e.1 < d[v] {
                    d[v] = d[min_node] + e.1;
                    p[v] = min_node as isize;
                    ok[v] = 0;
                }
            }
        }

        // println!("# loop:{}", count);
        // for i in 0..N {
        //     println!(" edge:{} d:{} p:{} ok:{}", i, d[i], p[i], ok[i]);
        // }
        // count++;
    }

    println!("{}", d[N-1]);
}
