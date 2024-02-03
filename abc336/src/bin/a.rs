use proconio::input;
use std::convert::TryInto;

fn main() {
    input!{
        n: u32,
    }
    println!("L{}ng", "o".repeat(n.try_into().unwrap()));
}
