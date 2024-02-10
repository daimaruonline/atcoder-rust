#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        Q: usize,
        query: [[usize; 2]; Q],
    }
    let mut ans = Vec::new();
    for query in query {
        match query[0] {
            1 => {
                ans.push(query[1]);
            },
            2 => {
                let index = ans.len()-query[1];
                println!("{}", ans[index as usize]);
            },
            _ => todo!(),
        }
    }
}
