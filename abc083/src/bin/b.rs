use proconio::input;

fn main() {
    input!{
        n: u32,
        a: u32,
        b: u32,
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut sum = 0;
        let mut _i = i;
        while _i > 0 {
            sum += _i % 10;
            _i = _i / 10;
        }
        if sum >= a && sum <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
