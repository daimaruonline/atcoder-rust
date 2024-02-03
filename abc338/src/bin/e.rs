use proconio::input;
use std::convert::TryInto;

fn main() {
    input!{
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut ab2 = vec![(0, 0); 2*n];
    let aaa = ((2 * n) - 1).try_into().unwrap();
    for e in &ab {
        // let diff = (e.0 - e.1).abs();
        // if diff == 1 || diff == aaa {
        //     // 隣接しているので交差しない→スキップ.
        //     continue;
        // }
        if e.0 < e.1 {
            // ab2.push((e.0, e.1));
            if (e.1 - e.0) == 1 || (e.1 - e.0) == aaa {
                continue;
            }
            ab2[e.0] = (e.0, e.1);
        } else {
            // ab2.push((e.1, e.0));
            if (e.0 - e.1) == 1 || (e.0 - e.1) == aaa {
                continue;
            }
            ab2[e.1] = (e.1, e.0);
        }
        // println!("{} {}", e.0, e.1);
        // println!("{:?}", e);
    }
    // ab2.sort_by(|a, b| a.0.cmp(&b.0));

    let mut ans = false;
    let max_len = ab2.len();
    'outer: for i in 0..(2*n) {
        let e1 = &ab2[i];
        if *e1 == (0, 0) {
            continue;
        }
        for j in (i+1)..(2*n) {
            let e2 = &ab2[j];
            if *e2 == (0, 0) {
                continue;
            }
            // println!("{} {} {} {}", e1.0, e1.1, e2.0, e2.1);
            if e1.1 < e2.0 {
                break;
            }
            if e2.0 < e1.1 && e1.1 < e2.1 {
            // if e1.1 < e2.1 {
                ans = true;
                break 'outer;
            }
        }
    }
    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}


