use std::collections::VecDeque;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut q = VecDeque::from(a);
    for _ in 0..k {
        q.pop_front();
        q.push_back(0);
    }
    println!("{}", q.iter().join(" "));
}

fn yesno(is_ok: bool) {
    if is_ok {
        println!("Yes")
    } else {
        println!("No")
    }
}
