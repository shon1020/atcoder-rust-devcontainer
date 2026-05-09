use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;
fn main() {
    input! {
        n: usize,
        m: usize,
        f: [Usize1; n],
    }

    let mut close = vec![false; m];
    let mut set = HashSet::new();
    let mut is_ok = true;

    for i in 0..n {
        close[f[i]] = true;
        if set.contains(&f[i]) {
            is_ok = false;
        } else {
            set.insert(&f[i]);
        }
    }

    let mut close_ok = true;
    for i in 0..m {
        if !close[i] {
            close_ok = false;
        }
    }
    println!("{}", if is_ok { "Yes" } else { "No" });
    println!("{}", if close_ok { "Yes" } else { "No" });
}
