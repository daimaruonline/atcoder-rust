use proconio::input;

fn main() {
    input!{
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }
    let mut cnt = 0;
    for ic in 0..=c {
        for ib in 0..=b {
            for ia in 0..=a {
                let sum = ia * 500 + ib * 100 + ic * 50;
                if sum == x {
                    cnt += 1;
                } else if sum > x {
                    break;
                }
            }
        }
    }
    println!("{}", cnt);
}

