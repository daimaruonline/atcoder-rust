#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;
use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    input!{
        _N: usize,
        mut S: Chars,
        Q: usize,
    }
    let mut rules: HashMap<char, char> = HashMap::new();
    for _i in 0..Q {
        input!{
            c: char,
            d: char,
        }
        if c == d {
            continue;
        }
        rules.entry(c).or_insert(d);
        let mut change_list: Vec<(char, char)> = Vec::new();
        for rule in &rules {
            if rule.1 == &c {
                change_list.push((*rule.0, d));
            }
        }
        for e in change_list {
            rules.insert(e.0, e.1);
        }

        // for rule in &rules {
        //     println!("{} -> {}", rule.0, rule.1);
        // }
        // println!("========");
    }
    let mut ans: Vec<char> = Vec::new();
    for c in S {
        if let Some(val) = rules.get(&c) {
            ans.push(*val);
        } else {
            ans.push(c);
        }
    }
    println!("{}", ans.iter().join(""));
}
