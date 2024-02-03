use proconio::input;
// use proconio::marker::Chars;

fn main() {
    input!{
        s: String,
    }
    let r = fnc(&s[..]);
    if r > 0 {
        println!("{}", "YES");
    } else {
        println!("{}", "NO");
    }
    // println!("{}", r > 0 ? "YES" : "NO");
}

fn fnc(s: &str) ->i32 {
    if s.is_empty() {
        return 1;
    }

    if s.len() >= "dreamer".len() {
        let mut f = true;
        for (i, x) in "dreamer".chars().enumerate() {
            if s.chars().nth(i).unwrap() != x {
                f = false;
                break;
            }
        }
        if f {
            if fnc(&s["dreamer".len()..]) > 0 {
                return 1;
            }
        }
    }
    if s.len() >= "eraser".len() {
        let mut f = true;
        for (i, x) in "eraser".chars().enumerate() {
            if s.chars().nth(i).unwrap() != x {
                f = false;
                break;
            }
        }
        if f {
            if fnc(&s["eraser".len()..]) > 0 {
                return 1;
            }
        }
    }
    if s.len() >= "dream".len() {
        let mut f = true;
        for (i, x) in "dream".chars().enumerate() {
            if s.chars().nth(i).unwrap() != x {
                f = false;
                break;
            }
        }
        if f {
            if fnc(&s["dream".len()..]) > 0 {
                return 1;
            }
        }
    }
    if s.len() >= "erase".len() {
        let mut f = true;
        for (i, x) in "erase".chars().enumerate() {
            if s.chars().nth(i).unwrap() != x {
                f = false;
                break;
            }
        }
        if f {
            if fnc(&s["erase".len()..]) > 0 {
                return 1;
            }
        }
    }

    return -1;
}