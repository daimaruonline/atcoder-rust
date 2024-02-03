use proconio::input;

fn main() {
    input!{
        n: u32,
        ar: [i32; n],
    }
    let mut total: i64 = 0;
    let mut min: i64 = 0;
    for _i in 0..n {
        total += ar[_i as usize] as i64;
        if total < min {
            min = total;
        }
    }
    println!("{}", total + -min);
}
