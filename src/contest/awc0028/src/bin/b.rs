use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        t: [usize; n],
    }

    let mut cnt = 0;
    let mut ans = 0;
    let mut is_ok = false;

    for i in 0..n {
        if l <= t[i] && t[i] <= r {
            if is_ok {
                cnt += 1;
            } else {
                cnt += 1;
                is_ok = true;
            }
        } else {
            if is_ok {
                ans = max(ans, cnt);
                cnt = 0;
                is_ok = false;
            }
        }
    }
    if is_ok {
        ans = max(ans, cnt);
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
