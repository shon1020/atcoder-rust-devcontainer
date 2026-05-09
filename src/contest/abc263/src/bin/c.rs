use std::default;

use itertools::Itertools;
use proconio::input;
//やり直し
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    for perm in (1..=m).combinations(n) {
        println!("{}", perm.iter().join(" "));
    }
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
