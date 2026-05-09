use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); n],
    }

    let mut bumon_pre = vec![0; m];
    let mut bumon_now = vec![0; m];

    for (a, b) in ab {
        bumon_pre[a] += 1;
        bumon_now[b] += 1;
    }

    for i in 0..m {
        println!("{}", bumon_now[i] - bumon_pre[i]);
    }
}
fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
