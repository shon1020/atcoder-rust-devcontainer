use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
    }

    let mut ans_vec = vec![vec!['#'; w]; h];

    for i in 0..h {
        for j in 0..w {
            if i != 0 && i != h - 1 && j != 0 && j != w - 1 {
                ans_vec[i][j] = '.';
            }
        }
    }

    for ans in ans_vec {
        println!("{}", ans.iter().join(""));
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
