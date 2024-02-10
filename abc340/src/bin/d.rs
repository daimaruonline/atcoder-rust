#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input!{
        N: usize,
        ABX: [[usize; 3]; N-1],
    }
    let mut keisan: Vec<usize> = vec![9999999999; N+1];
    keisan[1] = 0;
    for (i, abx) in ABX.iter().enumerate() {
        let stage = i+1;
        // println!("stage:{}", stage);
        if keisan[stage+1] > (keisan[stage] + abx[0]) {
            keisan[stage+1] = keisan[stage] + abx[0];
        }

        if keisan[abx[2]] > (keisan[stage] + abx[1]) {
            keisan[abx[2]] = keisan[stage] + abx[1];
        }
        for (i, keisan) in keisan.iter().enumerate() {
            println!("{} {}", i, keisan);
        }
    }
    println!("{}", keisan[N]);
}
