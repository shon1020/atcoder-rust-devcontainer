use itertools::Itertools;
use proconio::marker::Chars;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n],
        m: usize,
    }

    let mut cad_vec = vec![HashSet::new(); n];
    let mut s_vec = Vec::new();
    for i in 0..m {
        input! {
            s: Chars,
        }
        s_vec.push(s.clone());

        for i in 0..n {
            let a = ab[i].0;
            let b = ab[i].1;
            if s.len() == (a + 1) {
                cad_vec[i].insert(s[b]);
            }
        }
    }
    for an in s_vec {
        let mut is_ok = true;
        if an.len() == n {
            for i in 0..n {
                if !cad_vec[i].contains(&an[i]) {
                    is_ok = false;
                }
            }
            yesno(is_ok)
        } else {
            println!("No");
        }
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
