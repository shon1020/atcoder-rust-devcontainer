use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        t: f64,
        cs: [(Usize1, f64); m],
    }
    let mut p_vec = vec![0.0; n];
    let mut count = vec![0.0; n];

    for (c, s) in cs {
        p_vec[c] += s;
        count[c] += 1.0;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        if count[i] == 0.0 {
            continue;
        }
        if (p_vec[i] / count[i]) < t {
            ans += 1;
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
