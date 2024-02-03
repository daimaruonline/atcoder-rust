use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        s: Chars,
    }
    if !s[0].is_uppercase() {
        println!("No");
        return;
    }
    if s.len() == 1 {
        println!("Yes");
        return;
    }
    let mut ans = true;
    for c in &s[1..] {
        if c.is_uppercase() {
            ans = false;
            break;
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
