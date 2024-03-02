#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        S: Chars,
    }
    let mut a: Vec<(usize, usize)> = Vec::new();
    a.push((0, 0));
    a.push((0, 0));
    let mut c1 = 'A';
    let mut c2 = 'A';
    for (i, c) in S.iter().enumerate() {
        if c1 == *c || c1 == 'A' {
            a[0].0 += 1;
            if c1 == 'A' {
                c1 = *c;
                a[0].1 = i+1;
            }
        } else {
            a[1].0 += 1;
            if c2 == 'A' {
                c2 = *c;
                a[1].1 = i+1;
            }
        }
        if a[0].0 > 0 && a[1].0 > 0 && a[0].0 + a[1].0 > 2 {
            break;
        }
        // println!("{} {} {} {} {} {}", c1, a[0].0, a[0].1, c2, a[1].0, a[1].1);
    }
    if a[0].0 < a[1].0 {
        println!("{}", a[0].1);
    } else {
        println!("{}", a[1].1);
    }
}
