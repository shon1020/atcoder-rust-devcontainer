use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::{HashMap, HashSet};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: usize,
        c: [Usize1; k],
        uv: [(Usize1, Usize1); m],
    }

    let mut is_ok_vec = vec![false; n];
    let mut set = HashSet::new();
    let mut map = HashMap::new();
    for idx in c {
        is_ok_vec[idx] = true;
    }

    let mut ans: usize = is_ok_vec.iter().map(|&b| if b { 1 } else { 0 }).sum();

    println!("{}", ans);
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
