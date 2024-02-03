use proconio::input;

fn main() {
    input!{
        n: u8,
        ar: [String; n],
    }
    // for _i in 0..n {
    //     println!("{}", ar[_i as usize]);
    // }
    let mut p1: (u8, u8) = (0, 0);
    let mut p2: (u8, u8) = (0, 0);
    let mut mode = 0;
    'outer: for i in 0..n {
        if let Some(result) = ar[i as usize].find('P') {
            match mode {
                0 => { p1 = (i, result as u8); mode = 1; },
                1 => { p2 = (i, result as u8); break 'outer; },
                2_u8..=u8::MAX => todo!(),
            }
        }
    }
    println!("p1:{:?} p2:{:?}", p1, p2);
    let mut ans = -1;
    println!("{}", ans);
}

fn solve() {

}