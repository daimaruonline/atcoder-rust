use proconio::input;

fn main() {
    input!{
        n: u8,
        mut a: [i32; n],
    }
    let cnt = fnc(&mut a, 0);

    println!("{}", cnt);
}

fn fnc(array: &mut Vec<i32>, cnt: i32) -> i32 {
    for i in &mut *array {
        if *i % 2 != 0 {
            return cnt;
        }
        *i /= 2;
    }
    return fnc(array, cnt + 1);
}