use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s: Chars,
    }
    let mut sorted_s = s.clone();
    sorted_s.sort();
    let mut ans = "Yes";
    for i in 0..s.len() {
        if s[i] != sorted_s[i] {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}
