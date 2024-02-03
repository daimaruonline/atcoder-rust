use proconio::input;
use proconio::marker::Bytes;
use std::convert::TryInto;

fn main() {
    input!{
        s: Bytes,
    }
    let mut ar:[u8; 128] = [0; 128];
    for c in s {
        ar[c as usize] += 1;
    }
    let mut ans = (0, 0);
    for (i, val) in ar.iter().enumerate() {
        if ans.1 < *val {
            ans.0 = i;
            ans.1 = *val;
        }
    }
    let mut aaa: Vec<u8> = Vec::new();
    aaa.push(ans.0.try_into().unwrap());
    println!("{}", String::from_utf8(aaa).unwrap());
}
