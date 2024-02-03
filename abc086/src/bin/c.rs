use proconio::input;

fn main() {
    input!{
        n: u32,
        plan: [(i32, i32, i32); n],
    }
    let mut f = true;
    let mut before = (0, 0, 0);
    for p in plan {
        let elapse = p.0 - before.0;
        let dis = (p.1 - before.1).abs() + (before.2 - p.2).abs();
        if !(dis == elapse || (elapse > dis && (dis - elapse) % 2 == 0)) {
            f = false;
            break;
        }
        before = p;
    }
    if f {
        println!("Yes");
    } else {
        println!("No");
    }
}