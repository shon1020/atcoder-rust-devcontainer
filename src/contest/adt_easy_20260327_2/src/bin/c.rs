use std::process;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        p: usize,
        mut l: [usize; n],
    }

    let mut count = 0;
    for i in 0..n {
        if l[i] >= t {
            count += 1;
        }
    }
    let mut ans = 0;
    if count >= p {
        println!("{}", ans);
        process::exit(0);
    }

    while count < p {
        ans += 1;
        for i in 0..n {
            if l[i] < t {
                l[i] += 1;
                if l[i] == t {
                    count += 1;
                }
            } else {
                l[i] += 1;
            }
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
