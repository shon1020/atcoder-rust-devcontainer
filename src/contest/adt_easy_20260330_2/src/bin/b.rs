use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    for i in 0..256 {
        if a ^ i == b {
            println!("{}", i);
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
