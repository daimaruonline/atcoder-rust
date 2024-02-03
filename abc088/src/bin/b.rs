use proconio::input;

fn main() {
    input!{
        n: u32,
        mut a: [u32; n],
    }

    a.sort();
    a.reverse();
    let mut alice = 0;
    let mut bob = 0;
    for i in 0..n {
        if i % 2 == 0 {
            alice += a[i as usize];
        } else {
            bob += a[i as usize];
        }
    }
    println!("{}", alice - bob);
}
