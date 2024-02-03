use proconio::input;
use std::convert::TryInto;

fn main() {
    input!{
        mut n: u64,
    }
    let array = [0, 2, 4, 6, 8];
    n -= 1;
    let mut index : usize = (n % 5).try_into().unwrap();
    let mut ans: u64 = array[index];
    n = n / 5;
    let mut digits = 10;
    while n > 0 {
        index = (n % 5).try_into().unwrap();
        ans += array[index] * digits;
        n = n / 5;
        digits *= 10;
        // println!("{}", ans);
    }
    println!("{}", ans);
}
// 8 % 5 = 3 -> 4
// 8 / 5 = 1
// 1 % 5 = 1 -> 0

// 11 % 5 = 1 -> 0
// 11 / 5 = 2
// 2 % 5 = 2 -> 2
// 2 / 5 = 0
