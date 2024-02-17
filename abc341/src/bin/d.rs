#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        M: usize,
        K: usize,
    }

    // let mut count = 0;
    // let mut next = std::cmp::min(N, M);
    // loop {
    //     let mod_n = next % N;
    //     let mod_m = next % M;
    //     if (mod_n == 0 && mod_m != 0) || (mod_n != 0 && mod_m == 0) {
    //         count += 1;
    //         if count == K {
    //             println!("{}", next);
    //             break;
    //         }
    //     }
    //     // println!("{} {} {} {}", next, next+std::cmp::min(N-mod_n, M-mod_m), mod_n, mod_m);
    //     next += std::cmp::min(N-mod_n, M-mod_m);
    // }

    let lcm = (N * M) / gcd(N, M);
    let mut l = 0;
    let mut r = usize::max_value();
    while (l+1) < r {
        // println!("{}, {}", l, r);
        let mid = (l + r) / 2;
        let y = (mid/N) + (mid/M) - 2 * (mid/lcm);
        if y < K {
            l = mid;
        } else {
            r = mid;
        }
    }
    println!("{}", r);
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    if x > y {
        std::mem::swap(&mut x, &mut y);
    }
    if y % x == 0 {
        return x;
    }
    return gcd(y%x, x);
}
