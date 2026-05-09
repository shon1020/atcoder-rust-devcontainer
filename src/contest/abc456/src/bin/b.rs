use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: [[usize; 6]; 3],
    }
    let mut set = HashSet::new();
    set.insert((4, 5, 6));
    set.insert((4, 6, 5));
    set.insert((5, 4, 6));
    set.insert((5, 6, 4));
    set.insert((6, 5, 4));
    set.insert((6, 4, 5));
    let mut count: f64 = 0.0;
    for i in 0..6 {
        for j in 0..6 {
            for k in 0..6 {
                if set.contains(&(a[0][i], a[1][j], a[2][k])) {
                    count += 1.0;
                }
            }
        }
    }
    println!("{}", count / 216.0);
}
