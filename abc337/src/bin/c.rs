use proconio::input;
use itertools::Itertools;
// use std::convert::TryInto;

fn main() {
    input!{
        n: u32,
        a: [i32; n],
    }
    let mut b: Vec<u32> = vec![n; n as usize];
    let mut front = 0;
    for i in 0..n {
        let val = a[i as usize] - 1;
        if val < 0 {
            front = i;
        } else {
            b[val as usize] = i;
        }
        // println!("{}", b.iter().join(" "));
    }
    let mut c: Vec<u32> = Vec::new();
    while front < n {
        c.push(front + 1);
        front = b[front as usize];
    }
    println!("{}", c.iter().join(" "));
   
    // let mut ans_s:String = String::new();
    // while front < n {
    //     ans_s.push_str(&front.to_string());
    //     ans_s.push_str(" ");
    //     front = ans[front as usize];
    // }
    // println!("{}", ans_s.trim());

    // let mut ans: Vec<(i32, u32)> = Vec::new();
    // for (i, val) in a.iter().enumerate() {
    //     ans.push((*val, (i + 1).try_into().unwrap()));
    // }
    // ans.sort_by(|a, b| a.0.cmp(&b.0));
    // let mut ans_s:String = String::new();
    // for val in ans {
    //     ans_s.push_str(&val.1.to_string());
    //     ans_s.push_str(" ");
    // }
    // println!("{}", ans_s.trim());
    // for (i, val) in a.iter().enumerate() {
    //     if val == -1 {
    //         ans.insert(0, i + 1);
    //         continue;
    //     }

    // }
    // let mut search: i32 = -1;
    // for _i in 0..n {
    //     for (i, e) in a.iter().enumerate() {
    //         if e == &search {
    //             ans.push((i + 1).try_into().unwrap());
    //             search = i as i32 + 1;
    //             break;
    //         }
    //     }
    // }
    // println!("{}", ans.iter().join(" "));
}
