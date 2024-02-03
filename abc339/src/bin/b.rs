use proconio::input;

fn main() {
    input!{
        h: u8,
        w: u8,
        n: u16,
    }
    let mut g: Vec<Vec<char>> = vec![vec!['.'; w as usize]; h as usize];
    let mut now: (u8, u8) = (0, 0);   // h, w
    let mut dir = 0;
    for _i in 0..n {
        let t = &mut g[now.0 as usize][now.1 as usize];
        let new_pos_dir = get_new_pos_dir(&now, &dir, t, &h, &w);
        if t == &'.' {
            // 現在いるマスを黒に塗り替え、時計回りに 90∘回転し、向いている方向に 1 マス進む.
            *t = '#';
        } else {
            // 現在いるマスを白に塗り替え、反時計回りに 90∘回転し、向いている方向に 1 マス進む.
            *t = '.';
        }
        now = new_pos_dir.0;
        dir = new_pos_dir.1;

        // if _i < 5 {
        //     for i in 0..h {
        //         let mut line: String = "".to_string();
        //         for j in 0..w {
        //             line.push(g[i as usize][j as usize]);
        //         }
        //         println!("{}", line);
        //     }
        //     println!("");
        // }
    }
    for i in 0..h {
        let mut line: String = "".to_string();
        for j in 0..w {
            line.push(g[i as usize][j as usize]);
        }
        println!("{}", line);
    }
    // println!("{}", g.len());
}

fn get_new_pos_dir(pos: &(u8, u8), dir: &u8, col: &char, h: &u8, w: &u8) -> ((u8, u8), u8) {
    let mut new_dir = *dir;
    if col == &'.' {
        new_dir = (new_dir + 1) % 4;
    } else {
        new_dir = (new_dir + 4 - 1) % 4;
    }
    let mut new_pos = *pos;
    match new_dir {
        0 => { new_pos = ((new_pos.0 + h - 1) % h, new_pos.1); },
        1 => { new_pos = (new_pos.0, (new_pos.1 + 1) % w); },
        2 => { new_pos = ((new_pos.0 + 1) % h, new_pos.1); },
        3 => { new_pos = (new_pos.0, (new_pos.1 + w - 1) % w); },
        4_u8..=u8::MAX => todo!()
    }
    return (new_pos, new_dir);
}
