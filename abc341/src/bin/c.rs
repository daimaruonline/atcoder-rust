#![allow(non_snake_case)]
use std::collections::HashMap;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        H: usize,
        W: usize,
        N: usize,
        T: Chars,
        S: [Chars; H],
    }
    let mut dir: HashMap<char, (i8, i8)> = HashMap::new();
    dir.insert('L', (0, -1));
    dir.insert('R', (0, 1));
    dir.insert('U', (-1, 0));
    dir.insert('D', (1, 0));
    let mut res = 0;
    for i in 1..H {
        for j in 1..W {
            if S[i][j] == '#' {
                continue;
            }
            let mut goal = true;
            let mut cur: (i64, i64) = (i as i64, j as i64);
            for t in &T {
                cur = (cur.0+dir[&t].0 as i64, cur.1+dir[&t].1 as i64);
                if S[cur.0 as usize][cur.1 as usize] == '#' {
                    goal = false;
                    break;
                }
            }
            if goal == true {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
