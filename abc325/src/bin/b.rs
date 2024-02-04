#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        WX: [(usize, usize); N],
    }
    // println!("{}", WX.len());
    // for &wx in &WX {
    //     println!("{} {}", wx.0, wx.1);
    // }
    let mut result = 0;
    for t in 0..24 {
        let mut sum = 0;
        for &wx in &WX {
            let x = (t + wx.1) % 24;
            if x < 9 || x >= 18 {
                continue;
            }
            sum += wx.0;
        }
        if result < sum {
            result = sum;
        }
        // println!("{} {}", t, sum);
    }
    println!("{}", result);
}
