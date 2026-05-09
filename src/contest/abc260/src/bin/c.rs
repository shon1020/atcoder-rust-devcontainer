use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    }

    let mut ans: usize = rec(n, &mut true, x, y);
    println!("{}", ans);
}

fn rec(level: usize, is_red: &mut bool, x: usize, y: usize) -> usize {
    if level == 1 {
        if *is_red {
            return 0;
        } else {
            return 1;
        }
    }
    if *is_red {
        return rec(level - 1, &mut true, x, y) + rec(level, &mut false, x, y) * x;
    } else {
        return rec(level - 1, &mut true, x, y) + rec(level - 1, &mut false, x, y) * y;
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
