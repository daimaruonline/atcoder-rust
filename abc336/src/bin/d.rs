use proconio::input;

fn main() {
    input!{
        n: u32,
        mut a: [u32; n],
    }
    // let k = a.sort().reverse()[0];
    let mut ans = a.len() / 2;
    if a[0] != 1 {
        a[0] = 1;
    }
    if a[-1] != 1 {
        a[-1] = 1;
    }
    let before = 1;
    for elm in a[1..] {
        if elm > before {
            elm = before;
        } else {

        }
    }
    println!("{}", ans);
}
