use proconio::input;

fn main() {
    input!{
        n: u32,
        a: [[u32; 2]; n],
    }
    let mut sum_t = 0;
    let mut sum_a = 0;
    for e in a {
        sum_t += e[0];
        sum_a += e[1];
    }
    if sum_t > sum_a {
        println!("Takahashi");
    } else if sum_t == sum_a {
        println!("Draw");
    } else {
        println!("Aoki");
    }
}
