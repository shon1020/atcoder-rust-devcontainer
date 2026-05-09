use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::cmp::max;
fn main() {
    input! {
        n: Chars,
    }

    let mut ans = 0;

    for i in 0..1 << n.len() {
        let mut s = Vec::new();
        let mut t = Vec::new();

        for j in 0..n.len() {
            if (i >> j & 1) == 1 {
                s.push(n[j]);
            } else {
                t.push(n[j]);
            }
        }
        s.sort();
        t.sort();
        let s = s
            .iter()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0);
        let t = t
            .iter()
            .rev()
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0);
        ans = max(ans, s * t);
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
