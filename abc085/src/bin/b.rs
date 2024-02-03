use proconio::input;

fn main() {
    input!{
        n: u32,
        mut a: [u32; n],
    }

    a.sort();
    a.dedup();
    println!("{}", a.len());
}
