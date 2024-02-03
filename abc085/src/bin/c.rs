use proconio::input;

fn main() {
    input!{
        n: u32,
        y: u32,
    }
    for i_10000 in 0..=n {
        for i_5000 in 0..=(n-i_10000) {
            let i_1000 = n-i_10000-i_5000;
            if (i_10000*10000 + i_5000*5000 + i_1000*1000) == y {
                println!("{} {} {}", i_10000, i_5000, i_1000);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}
