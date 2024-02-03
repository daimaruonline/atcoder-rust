// -*- coding:utf-8-unix -*-

use proconio::input;

// A - Welcome to AtCoder
// https://atcoder.jp/contests/practice/tasks/practice_1

fn main() {
    input! {
        mut a: i32,
        mut b: i32,
        mut c: i32,
        mut s: String,
    }
    let sum = a + b + c;
    println!("{} {}", sum, s);
}
