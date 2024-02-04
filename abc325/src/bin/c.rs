use proconio::input;
use proconio::marker::Chars;

const DIR: [(i32, i32); 8] = [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
// const DIR: [(i32, i32); 4] = [(0, 1), (1, -1), (1, 0), (1, 1)];

fn main() {
    input!{
        h: i32,
        w: i32,
        mut s: [Chars; h],
    }
    let mut ans = 0;
    let mut stack: Vec<(i32, i32)> = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if s[i as usize][j as usize] == '#' {
                stack.push((i, j));
                ans += 1;

                while !stack.is_empty() {
                    let f = stack.pop().unwrap();
                    s[f.0 as usize][f.1 as usize] = 'c';
                    for d in DIR {
                        let p = (f.0 + d.0, f.1 + d.1);
                        if (p.0 < 0 || p.0 >= h) || (p.1 < 0 || p.1 >= w) {
                            continue;
                        }
                        if s[p.0 as usize][p.1 as usize] == '#' {
                            stack.push(p);
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
