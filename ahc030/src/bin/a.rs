#![allow(non_snake_case)]
use proconio::{input, source::line::LineSource};
use std::io::{stdin, stdout, BufReader, Write};
use std::io::BufRead;

static MODE: i32 = 1;
static DEBUG: i32 = 1;

fn main() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
 
    input!{
        from &mut source,
        N: usize,
        M: usize,
        eps: f64,
    }
    let mut fields: Vec<Vec<(usize, usize)>> = Vec::with_capacity(M);
    for _M in 0..M {
        input! {
            from &mut source,
            d: usize,
            ij: [(usize, usize); d],
        }
        fields.push(ij);
    }
    match MODE {
        1 => {
            solve1(&mut source, N, M, eps, &fields);
        },
        2 => {
            solve2(&mut source, N, M, eps, &fields);
        }
        _ => todo!(),
    }

    input! {
        from &mut source,
        _resp: usize,
    }
}

fn solve2<R: BufRead>(source: &mut LineSource<R>, N: usize, M: usize, eps: f64, _fields: & Vec<Vec<(usize, usize)>>) {
    let mut has_oil: Vec<(usize, usize)> = Vec::new();
    solve2_sub(source, 0, 0, N/2, N/2, M, eps, &mut has_oil);
    solve2_sub(source, 0, N/2, N/2, N, M, eps, &mut has_oil);
    solve2_sub(source, N/2, 0, N, N/2, M, eps, &mut has_oil);
    solve2_sub(source, N/2, N/2, N, N, M, eps, &mut has_oil);

    let formatted_string: Vec<String> = has_oil.iter()
    .map(|&(a, b)| format!("{} {}", a, b))
    .collect();
    println!("a {} {}", has_oil.len(), formatted_string.join(" "));
    stdout().flush().unwrap();
}
fn solve2_sub<R: BufRead>(source: &mut LineSource<R>, x0: usize, y0: usize, x1: usize, y1: usize, M: usize, eps: f64, has_oil: &mut Vec<(usize, usize)>) {
    if x0 == x1 || y0 == y1 {
        return;
    }
    if DEBUG == 1 {
        println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {})", x0, y0, x1, y1);
        stdout().flush().unwrap();
    }

    if (x1-x0)==1 && (y1-y0)==1 {
        println!("q 1 {} {}", x0, y0);
        stdout().flush().unwrap();
        input! {
            from &mut *source,
            resp: usize,
        }
        if resp != 0 {
            has_oil.push((x0, y0));
        }
    } else {

        print!("q {}", (x1-x0)*(y1-y0));
        for i in x0..x1 {
            for j in y0..y1 {
                print!(" {} {}", i, j);
            }
        }
        println!("");
        stdout().flush().unwrap();
        input! {
            from &mut *source,
            x: usize,
        }
        if DEBUG == 1 {
            println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {}) x={}", x0, y0, x1, y1, x);
            stdout().flush().unwrap();
        }

        if x == 0 {
            if DEBUG == 1 {
                println!("# skip");
                stdout().flush().unwrap();
                for i in x0..x1 {
                    for j in y0..y1 {
                        println!("#c {} {} #b0b0b0", i, j);
                        stdout().flush().unwrap();
                    }
                }    
            }
            return;
        }

        if DEBUG == 1 {
            println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {}) => (x0, y0)=({}, {}) (x1, y1)=({}, {})", x0, y0, x1, y1, x0, y0, x0+(x1-x0)/2, y0+(y1-y0)/2);
            stdout().flush().unwrap();
            println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {}) => (x0, y0)=({}, {}) (x1, y1)=({}, {})", x0, y0, x1, y1, x0, y0+(y1-y0)/2, x0+(x1-x0)/2, y1);
            stdout().flush().unwrap();
            println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {}) => (x0, y0)=({}, {}) (x1, y1)=({}, {})", x0, y0, x1, y1, x0+(x1-x0)/2, y0, x1, y0+(y1-y0)/2);
            stdout().flush().unwrap();
            println!("# (x0, y0)=({}, {}) (x1, y1)=({}, {}) => (x0, y0)=({}, {}) (x1, y1)=({}, {})", x0, y0, x1, y1, x0+(x1-x0)/2, y0+(y1-y0)/2, x1, y1);
            stdout().flush().unwrap();
        }
        solve2_sub(source, x0,           y0,           x0+(x1-x0)/2, y0+(y1-y0)/2, M, eps, has_oil);
        solve2_sub(source, x0,           y0+(y1-y0)/2, x0+(x1-x0)/2, y1,           M, eps, has_oil);
        solve2_sub(source, x0+(x1-x0)/2, y0,           x1,           y0+(y1-y0)/2, M, eps, has_oil);
        solve2_sub(source, x0+(x1-x0)/2, y0+(y1-y0)/2, x1,           y1,           M, eps, has_oil);
    }
}

fn solve1<R: BufRead>(source: &mut LineSource<R>, N: usize, _M: usize, _eps: f64, fields: & Vec<Vec<(usize, usize)>>) {
    let mut total_oil = 0;
    for field in fields {
        total_oil += field.len();
    }

    let mut has_oil: Vec<(usize, usize)> = Vec::new();
    let mut found_oil = 0;
    'outer: for i in 0..N {
        for j in 0..N {
            println!("q 1 {} {}", i, j);
            stdout().flush().unwrap();
            input! {
                from &mut *source,
                resp: usize,
            }
            if resp > 0 {
                has_oil.push((i, j));
                found_oil += resp;
                if found_oil >= total_oil {
                    break 'outer;
                }
            }
        }
    }
    let formatted_string: Vec<String> = has_oil.iter()
    .map(|&(a, b)| format!("{} {}", a, b))
    .collect();
    println!("a {} {}", has_oil.len(), formatted_string.join(" "));
    stdout().flush().unwrap();
}
