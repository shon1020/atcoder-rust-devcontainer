use ac_library::math;
use proconio::input;
use std::cmp::{max, min};
use std::process;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        mut a: [i64; n],
    }

    a.sort();
    let mut ans = 0;

    for i in 0..n {
        let l = a[0] * y - a[i] * x;
        if l >= 0 && l % (y - x) == 0 {
            ans += l / (y - x);
        } else {
            println!("-1");
            process::exit(0);
        }
    }

    println!("{}", ans);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
