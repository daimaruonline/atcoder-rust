use proconio::input;

fn main() {
    input!{
        s: String,
    }
    let mut r: u8 = 0;
    for c in s.chars() {
        if c == '1' {
            r += 1;
        }
    }
    println!("{}", r);
}
