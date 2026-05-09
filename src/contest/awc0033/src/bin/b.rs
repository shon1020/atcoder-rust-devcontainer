use either::Either::Left;
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        l: i64,
        r: i64,
        t: [i64; n],
    }

    let mut cad_vec: Vec<i64> = vec![0; n];
    for i in 0..n {
        if t[i] >= l && t[i] <= r {
            continue;
        } else if t[i] > r {
            cad_vec[i] = (t[i] - r).abs();
        } else {
            cad_vec[i] = (l - t[i]).abs();
        }
    }

    cad_vec.sort();

    let mut ans = 0;

    for i in 0..k {
        ans += cad_vec[i];
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
