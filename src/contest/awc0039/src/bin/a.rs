use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        pc: [(Usize1, char); m],
    }

    let mut recover_vec = vec![' '; n];

    for (p, c) in pc {
        recover_vec[p] = c;
    }

    for i in 0..q {
        input! {
            t: Chars
        }
        let mut is_ok = true;
        for i in 0..n {
            if recover_vec[i] == ' ' {
                continue;
            }
            if recover_vec[i] != t[i] {
                is_ok = false;
            }
        }
        yesno(is_ok);
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
