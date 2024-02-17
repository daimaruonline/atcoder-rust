#![allow(non_snake_case)]
use proconio::input;
use itertools::Itertools;

fn main() {
    input!{
        N: usize,
        A: [usize; N],
    }
    let mut fl: Vec<usize> = vec![0; N+1];
    let mut s: Vec<usize> = Vec::new();
    let mut v: i64 = 1;
    while fl[v as usize] == 0 {
        fl[v as usize] = 1;
        s.push(v as usize);
        v = A[(v - 1) as usize] as i64;
    }

    let mut res: Vec<usize> = Vec::new();
    for nx in s {
        if nx == v as usize {
            v = -1;
        }
        if v == -1 {
            res.push(nx);
        }
    }
    println!("{}", res.len());
    println!("{}", res.iter().join(" "));
    // let mut glaph: Vec<Vec<usize>> = vec![Vec::new(); N+1];
    // for i in 0..N {
    //     glaph[A[i]].push(i+1);
    // }
    // // for i in 1..=N {
    // //     println!("i:{} {}", i, glaph[i].iter().join(" "));
    // // }
    // for i in 1..=N {
    //     let mut route: Vec<usize> = Vec::new();
    //     let ans = solve(i, i, i, &glaph, &mut route);
    //     if ans != -1 {
    //         let r: Vec<_> = route.iter().rev().collect();
    //         println!("{}", route.len());
    //         println!("{}", r.iter().join(" "));
    //         break;
    //     }
    // }
}

// fn solve(now: usize, start: usize, goal: usize, glaph: &Vec<Vec<usize>>, route: &mut Vec<usize>) -> i32 {
//     // println!("n:{} s:{} g:{} r:{}", now, start, goal, route.iter().join(" "));

//     route.push(now);
//     for g in &glaph[now] {
//         if *g == goal {
//             return goal as i32;
//         }
//         let ans = solve(*g, start, goal, glaph, route);
//         if ans != -1 {
//             return ans;
//         }
//     }
//     route.pop();
//     return -1;
// }