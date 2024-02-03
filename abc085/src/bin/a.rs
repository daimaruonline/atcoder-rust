use proconio::input;

fn main() {
    input!{
        a: u32,
        b: u32,
    }
    let c: u32 = a * b;
    println!("{}", c);
}

