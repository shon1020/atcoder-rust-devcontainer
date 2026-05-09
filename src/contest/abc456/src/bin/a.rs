use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    let mut is_ok = false;
    for i in 1..=6 {
        for j in 1..=6 {
            for k in 1..=6 {
                if i + j + k == x {
                    is_ok = true;
                }
            }
        }
    }
    println!("{}", if is_ok { "Yes" } else { "No" });
}
