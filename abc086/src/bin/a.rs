use proconio::input;

fn main() {
    input!{
        a: u32,
        b: u32,
    }
    let c: u32 = a * b;
    if c % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
}