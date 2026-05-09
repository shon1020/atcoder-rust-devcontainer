use im_rc::HashSet;
use proconio::input;
use proconio::marker::Chars;

const D: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    input! {
        h: i64,
        w: i64,
        s: [Chars; h],
    }

    let mut seen: Vec<Vec<bool>> = vec![vec![false; w as usize]; h as usize];
    let mut ans = 0;

    for i in 1..h - 1 {
        for j in 1..w - 1 {
            if seen[i as usize][j as usize] {
                continue;
            }
            // if s[i as usize][j as usize] == '#' {
            //     continue;
            // }
            let mut is_ok = false;
            if s[i as usize][j as usize] == '.' && !dfs(&s, i, j, h, w, &mut seen, &mut is_ok) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn dfs(
    s: &Vec<Vec<char>>,
    h_idx: i64,
    w_idx: i64,
    h: i64,
    w: i64,
    seen: &mut Vec<Vec<bool>>,
    is_ok: &mut bool,
) -> bool {
    seen[h_idx as usize][w_idx as usize] = true;
    if h_idx == 0 || w_idx == 0 || h_idx == h - 1 || w_idx == w - 1 {
        *is_ok = true;
    }
    for (dx, dy) in &D {
        let dh = h_idx + dy;
        let dw = w_idx + dx;
        if dh >= h || dw >= w || dh < 0 || dw < 0 {
            continue;
        }
        if s[dh as usize][dw as usize] == '#' {
            continue;
        }
        if seen[dh as usize][dw as usize] {
            continue;
        }
        dfs(s, dh, dw, h, w, seen, is_ok);
    }
    *is_ok
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
