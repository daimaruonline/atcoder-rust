#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input!{
        N: usize,
    }
    let mut keisan: HashMap<usize, usize> = HashMap::new();
    keisan.insert(0, 0);
    keisan.insert(1, 0);
    keisan.insert(2, 2);
    keisan.insert(3, 5);
    let ans = solve(N, &mut keisan);
    println!("{}", ans);
}

fn solve(N: usize, keisan: &mut HashMap<usize, usize>) -> usize {
    match keisan.get(&N) {
        Some(&number) => {return number;},
        _ => {},
    }
    let mut ans = 0;
    if N % 2 == 0 {
        ans = N + solve(N/2, keisan) * 2;
    } else {
        ans = N + solve(N/2, keisan) + solve(N/2+1, keisan);
    }
    keisan.insert(N, ans);
    return ans;
}
